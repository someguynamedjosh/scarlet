use std::collections::HashSet;

use backtrace::Backtrace;
use itertools::Itertools;
use maplit::hashset;

use super::{InvariantSet, InvariantSetPtr, SetJustification, StatementJustifications};
use crate::{
    environment::Environment,
    item::{
        base::util::Stack, definitions::substitution::Substitutions, equality::Equal,
        util::unchecked_substitution, ItemPtr,
    },
    scope::{LookupInvariantError, LookupInvariantResult, Scope},
    shared::{indented, indented_with, TripleBool},
    util::{rcrc, PtrExtension},
};

const TRACE: bool = false;

pub type JustifyInvariantResult = Result<Vec<InvariantSetPtr>, LookupInvariantError>;

#[derive(Clone, Debug, PartialEq)]
pub struct JustifyStackFrame {
    base: ItemPtr,
    subs: Substitutions,
}

pub type JustifyStack = Vec<JustifyStackFrame>;

fn is_any_statement_justification_connected_to_root(
    justifications: &StatementJustifications,
) -> bool {
    for justification in justifications {
        let mut all_connected = true;
        for justification in justification {
            if !justification.borrow().connected_to_root {
                all_connected = false;
                break;
            }
        }
        if all_connected {
            return true;
        }
    }
    false
}

fn propogate_root_connectedness(of: &[InvariantSetPtr]) {
    loop {
        let mut progress = false;
        for set_ptr in of {
            let set = set_ptr.borrow();
            if set.connected_to_root {
                continue;
            }
            let mut all_statements_connected = true;
            if let Some(just) = &set.set_justification {
                for statement_justifications in just {
                    if !is_any_statement_justification_connected_to_root(statement_justifications) {
                        all_statements_connected = false;
                        break;
                    }
                }
            } else {
                all_statements_connected = false;
            }
            drop(set);
            if all_statements_connected {
                let mut set = set_ptr.borrow_mut();
                set.connected_to_root = true;
                progress = true;
            }
        }
        if !progress {
            break;
        }
    }
}

fn collect_invariant_sets(root: &ItemPtr) -> Vec<InvariantSetPtr> {
    let mut result = Vec::new();
    root.for_self_and_contents(&mut |item| {
        for inv_set in item.get_invariants() {
            result.push(inv_set);
        }
    });
    result
}

struct JustificationContext<'a> {
    stack: JustifyStack,
    sets: Vec<InvariantSetPtr>,
    env: &'a mut Environment,
}

impl Environment {
    pub fn justify_all(&mut self, root: &ItemPtr) {
        let all_sets = collect_invariant_sets(root);
        JustificationContext {
            stack: Vec::new(),
            sets: all_sets,
            env: self,
        }
        .justify_all()
    }

    pub fn justify(
        &mut self,
        root: &ItemPtr,
        context: &ItemPtr,
        statement: &ItemPtr,
        limit: u32,
    ) -> Result<StatementJustifications, LookupInvariantError> {
        let all_sets = collect_invariant_sets(root);
        JustificationContext {
            stack: Vec::new(),
            sets: all_sets,
            env: self,
        }
        .justify_statement(context, statement, limit)
    }
}

impl<'a> JustificationContext<'a> {
    fn justify_all(&mut self) {
        let mut encountered_err = false;
        const MAX_LIMIT: u32 = 4;
        for limit in 0..MAX_LIMIT {
            println!("{}/{}", limit, MAX_LIMIT);
            for set_ptr in self.sets.clone() {
                let set = set_ptr.borrow();
                if set.connected_to_root {
                    continue;
                }
                drop(set);
                let res = self.justify(&set_ptr, limit);
                let set = set_ptr.borrow();
                if limit == MAX_LIMIT - 1 && !set.connected_to_root && set.statements().len() > 0 {
                    if let Err(err) = res {
                        eprintln!("Error while justifying invariant set:");
                        eprintln!("{:?}", err);
                    } else {
                        eprintln!("The following can only be justified circularly:");
                        eprintln!("{:#?}", set);
                    }
                    encountered_err = true;
                }
            }
            propogate_root_connectedness(&self.sets);
            let mut all_connected = true;
            for set_ptr in &self.sets {
                let set = set_ptr.borrow();
                if !set.connected_to_root && set.required {
                    all_connected = false;
                }
            }
            if all_connected {
                break;
            } else if limit == MAX_LIMIT - 1 {
                eprintln!("Some invariants can only be justified circularly.");
                encountered_err = true;
            }
        }
        if encountered_err {
            todo!("nice error: Invariants are not justified.");
        }
    }

    fn justify(
        &mut self,
        set: &InvariantSetPtr,
        limit: u32,
    ) -> Result<SetJustification, LookupInvariantError> {
        let mut justifications = Vec::new();
        for required in set.borrow().justification_requirements() {
            let justified_by = self.justify_statement(&set.borrow().context, required, limit)?;
            justifications.push(justified_by);
        }
        set.borrow_mut().set_justification = Some(justifications.clone());
        Ok(justifications)
    }

    fn justify_statement(
        &mut self,
        context: &ItemPtr,
        statement: &ItemPtr,
        limit: u32,
    ) -> Result<StatementJustifications, LookupInvariantError> {
        let mut result = Vec::new();
        let ctx_scope = context.clone_scope();
        let available_invariant_sets = ctx_scope.get_invariant_sets();
        let iterate_over = available_invariant_sets;
        for other_set in iterate_over {
            for other_statement in other_set.borrow().statements() {
                if TRACE {
                    println!("Trying to link {:#?}", statement);
                    println!("by {:#?}", other_statement);
                }
                let eq = statement.get_equality(other_statement, limit);
                if TRACE {
                    println!("{:#?}", eq);
                }
                if let Ok(Equal::Yes(subs, _)) = eq {
                    if subs.len() > 0 {
                        continue;
                    }
                    result.push(vec![other_set.ptr_clone()]);
                    break;
                }
            }
        }
        match self.create_justification(context, statement, limit) {
            Ok(mut extra_invs) => result.append(&mut extra_invs),
            Err(err) => {
                if result.len() == 0 {
                    return Err(LookupInvariantError::MightNotExist);
                }
            }
        }
        Ok(result)
    }

    fn create_justification(
        &mut self,
        context: &ItemPtr,
        statement: &ItemPtr,
        limit: u32,
    ) -> Result<StatementJustifications, LookupInvariantError> {
        let mut err = LookupInvariantError::DefinitelyDoesNotExist;
        // let trace = statement.index == 833;
        let trace = false;
        if trace {
            println!("Trying to find justification of {:?}", statement);
        }
        if limit == 0 {
            if trace {
                println!("Limit reached.");
            }
            return Err(err);
        }
        let mut successful_candidates = Vec::new();
        for frame in self.stack.clone() {
            let subbed = unchecked_substitution(frame.base, &frame.subs);
            if let Equal::Yes(subs, rec) = statement.get_equality(&subbed, limit)? {
                if subs.len() > 0 {
                    continue;
                };
                if trace {
                    println!("Equal to a previous thing!");
                }
                // Deduplicate
                let rec: HashSet<_> = rec.into_iter().collect();
                let rec: Vec<_> = rec.into_iter().collect();
                if rec.len() != 1 {
                    return Err(LookupInvariantError::DefinitelyDoesNotExist);
                }
                let rec = &rec[0];
                let new_set = InvariantSet::new_recursive_justification(
                    context.ptr_clone(),
                    vec![rec.ptr_clone()].into_iter().collect(),
                );
                self.sets.push(new_set.ptr_clone());
                successful_candidates.push(vec![new_set]);
            }
        }
        let mut candidates = Vec::new();
        for at in self.env.auto_theorems.clone() {
            let invs_ptr = at.get_invariants()?;
            let invs = invs_ptr.borrow();
            for inv in invs.statements() {
                match inv.get_equality(&statement, limit - 1)? {
                    Equal::Yes(subs, _) => {
                        candidates.push((invs_ptr.ptr_clone(), inv.ptr_clone(), subs))
                    }
                    Equal::NeedsHigherLimit => err = LookupInvariantError::MightNotExist,
                    _ => (),
                }
            }
        }
        'check_next_candidate: for (inv_id, inv, subs) in candidates {
            if subs.len() == 0 {
                successful_candidates.push(vec![inv_id]);
                continue;
            }
            self.stack.push(JustifyStackFrame {
                base: inv.ptr_clone(),
                subs: subs.clone(),
            });
            let mut justifications = Vec::new();
            let ok = self.check_subs(
                context,
                statement,
                subs.clone(),
                limit,
                &mut justifications,
                &mut err,
                trace,
            );
            self.stack.pop();
            if !ok {
                continue 'check_next_candidate;
            }
            successful_candidates.push(justifications);
        }
        if successful_candidates.len() > 0 {
            Ok(successful_candidates)
        } else {
            Err(err)
        }
    }

    fn check_subs(
        &mut self,
        context: &ItemPtr,
        statement: &ItemPtr,
        subs: Substitutions,
        limit: u32,
        justifications: &mut Vec<InvariantSetPtr>,
        err: &mut LookupInvariantError,
        trace: bool,
    ) -> bool {
        let mut inv_subs = Substitutions::new();
        for (target, value) in subs {
            inv_subs.insert_no_replace(target.ptr_clone(), value);
            let target = target.borrow();
            for invv in target.invariants() {
                let statement = unchecked_substitution(invv.ptr_clone(), &inv_subs);
                if trace {
                    println!("Need to justify {:?}", statement);
                }
                let result = self.justify_statement(context, &statement, limit - 1);
                match result {
                    Ok(new_justifications) => {
                        if trace {
                            println!("Success!");
                        }
                        let set = rcrc(InvariantSet {
                            context: context.ptr_clone(),
                            statements: vec![statement.ptr_clone()],
                            set_justification: Some(vec![new_justifications]),
                            justification_requirements: vec![statement],
                            dependencies: hashset![],
                            required: false,
                            connected_to_root: false,
                        });
                        justifications.push(set);
                    }
                    Err(LookupInvariantError::Unresolved(..))
                    | Err(LookupInvariantError::MightNotExist) => {
                        if trace {
                            println!("{:?}", result);
                        }
                        *err = result.unwrap_err();
                        return false;
                    }
                    Err(LookupInvariantError::DefinitelyDoesNotExist) => {
                        if trace {
                            println!("Definitely unjustified");
                        }
                        return false;
                    }
                }
            }
        }
        true
    }
}
