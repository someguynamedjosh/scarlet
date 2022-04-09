use std::collections::HashSet;

use backtrace::Backtrace;
use itertools::Itertools;
use maplit::hashset;

use super::{InvariantSet, InvariantSetId};
use crate::{
    constructs::{substitution::Substitutions, Construct, GenInvResult},
    environment::{dependencies::DepResStackFrame, discover_equality::Equal, Environment, ItemId},
    scope::{LookupInvariantError, LookupInvariantResult, Scope},
    shared::{indented, indented_with, Id, TripleBool},
};

pub type JustifyInvariantResult = Result<Vec<InvariantSetId>, LookupInvariantError>;

#[derive(Clone, Debug)]
pub struct JustifyStackFrame {
    base: ItemId,
    subs: Substitutions,
}

pub type JustifyStack = Vec<JustifyStackFrame>;

pub type SetJustification = Vec<StatementJustifications>;
pub type StatementJustifications = Vec<StatementJustification>;
pub type StatementJustification = Vec<InvariantSetId>;

impl<'x> Environment<'x> {
    fn for_each_invariant_set(&mut self, mut operator: impl FnMut(&mut Self, InvariantSetId)) {
        let mut maybe_id = self.invariant_sets.first();
        while let Some(id) = maybe_id {
            operator(self, id);
            maybe_id = self.invariant_sets.next(id);
        }
    }

    fn is_any_statement_justification_connected_to_root(
        &self,
        justifications: &StatementJustifications,
    ) -> bool {
        for justification in justifications {
            let mut all_connected = true;
            for &id in justification {
                if !self.invariant_sets[id].connected_to_root {
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

    fn propogate_root_connectedness(&mut self) {
        loop {
            let mut progress = false;
            self.for_each_invariant_set(|env, id| {
                let set = &env.invariant_sets[id];
                if set.connected_to_root {
                    return;
                }
                let mut all_statements_connected = true;
                if let Some(just) = &set.statement_justifications {
                    for statement_justifications in just {
                        if !env.is_any_statement_justification_connected_to_root(
                            statement_justifications,
                        ) {
                            all_statements_connected = false;
                            break;
                        }
                    }
                } else {
                    all_statements_connected = false;
                }
                if all_statements_connected {
                    let set = &mut env.invariant_sets[id];
                    set.connected_to_root = true;
                    for &s in set.clone().statements() {
                        if s.index == 323 {
                            panic!();
                        }
                        let first = env.items.first().unwrap();
                    }
                    progress = true;
                }
            });
            if !progress {
                break;
            }
        }
    }

    pub(crate) fn justify_all(&mut self) {
        let mut encountered_err = false;
        for limit in 0..16 {
            self.for_each_invariant_set(|env, id| {
                if !env.invariant_sets[id].required {
                    return;
                }
                let res = env.justify(id, limit);
                if limit == 15
                    && !env.invariant_sets[id].connected_to_root
                    && env.invariant_sets[id].statements().len() > 0
                {
                    if let Err(err) = res {
                        eprintln!("Error while justifying invariant set:");
                        eprintln!("{:?}", err);
                    } else {
                        eprintln!("The following can only be justified recursively:");
                    }
                    println!("Statements:");
                    let first = env.items.first().unwrap();
                    for &statement in env.invariant_sets[id].clone().statements() {
                        println!("{}", env.show(statement, first));
                    }
                    // println!("Requires:");
                    // for &justification_requirement in
                    //     env.invariant_sets[id].clone().justification_requirements()
                    // {
                    //     println!("{}", env.show(justification_requirement, first));
                    // }
                    // println!("Available:");
                    // let ctx_scope = env.items[env.invariant_sets[id].context].scope.dyn_clone();
                    // let available_invariant_sets = ctx_scope.get_invariant_sets(env);
                    // let iterate_over = available_invariant_sets
                    //     .into_iter()
                    //     .map(|x| (x, env.invariant_sets[x].clone()))
                    //     .collect_vec();
                    // for (other_id, other_set) in iterate_over {
                    //     for &other_statement in other_set.clone().statements() {
                    //         println!("{}", env.show(other_statement, first));
                    //     }
                    // }
                    // eprintln!("Context:");
                    // let c = env.invariant_sets[id].context;
                    // eprintln!("{}", env.show(c, first));
                    encountered_err = true;
                }
            });
            self.propogate_root_connectedness();
            let mut all_connected = true;
            self.for_each_invariant_set(|env, id| {
                if !env.invariant_sets[id].connected_to_root && env.invariant_sets[id].required {
                    all_connected = false;
                }
            });
            if all_connected {
                break;
            } else if limit == 15 {
                // for (id, set) in self.invariant_sets.clone() {
                //     if !set.connected_to_root {
                //         println!("UNJUSTIFIED:")
                //     }
                //     println!("{:#?}", id);
                //     for &statement in &set.statements {
                //         println!("  statement:");
                //         println!("  {}", self.show(statement, statement));
                //     }
                //     for &just in &set.justification_requirements {
                //         println!("  requirement:");
                //         println!("  {}", self.show(just, just));
                //     }
                //     println!("{:#?}", set.justified_by());
                // }
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
        set_id: InvariantSetId,
        limit: u32,
    ) -> Result<SetJustification, LookupInvariantError> {
        let set = self.invariant_sets[set_id].clone();
        // if let Some(just) = set.statement_justifications {
        //     return Ok(just);
        // }
        let mut justifications = Vec::new();
        for &required in set.justification_requirements() {
            let justified_by = self.justify_statement(set.context, required, limit)?;
            justifications.push(justified_by);
        }
        self.invariant_sets[set_id].statement_justifications = Some(justifications.clone());
        Ok(justifications)
    }

    pub(super) fn justify_statement(
        &mut self,
        context: ItemId,
        statement: ItemId,
        limit: u32,
    ) -> Result<StatementJustifications, LookupInvariantError> {
        let mut result = Vec::new();
        let ctx_scope = self.items[context].scope.dyn_clone();
        let available_invariant_sets = ctx_scope.get_invariant_sets(self);
        let iterate_over = available_invariant_sets
            .into_iter()
            .map(|x| (x, self.invariant_sets[x].clone()))
            .collect_vec();
        for (other_id, other_set) in iterate_over {
            for &other_statement in other_set.clone().statements() {
                if let Ok(Equal::Yes(subs)) = self.discover_equal(statement, other_statement, limit)
                {
                    if subs.len() > 0 {
                        continue;
                    }
                    let first = self.items.first().unwrap();
                    result.push(vec![other_id]);
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
        context: ItemId,
        statement: ItemId,
        limit: u32,
    ) -> Result<StatementJustifications, LookupInvariantError> {
        let mut err = LookupInvariantError::DefinitelyDoesNotExist;
        let trace = false;
        if limit == 0 {
            return Err(err);
        }
        for frame in self.justify_stack.clone() {
            if let Equal::Yes(subs) = self.discover_equal_with_subs(
                statement,
                vec![],
                frame.base,
                vec![&frame.subs],
                limit,
            )? {
                if subs.len() > 0 {
                    continue;
                }
                let rec = self.evaluation_of_item_recurses_over(statement)?;
                if rec.len() == 0 {
                    continue;
                }
                let inv = self.push_invariant_set(InvariantSet::new_depending_on(
                    statement,
                    rec.into_iter().collect(),
                ));
                return Ok(vec![vec![inv]]);
            }
        }
        let mut candidates = Vec::new();
        for at in self.auto_theorems.clone() {
            let invs_id = self.generated_invariants(at);
            let invs = self.get_invariant_set(invs_id).clone();
            for &inv in invs.statements() {
                match self.discover_equal(inv, statement, limit - 1)? {
                    Equal::Yes(subs) => candidates.push((invs_id, inv, subs)),
                    Equal::NeedsHigherLimit => err = LookupInvariantError::MightNotExist,
                    _ => (),
                }
            }
        }
        let mut successful_candidates = Vec::new();
        'check_next_candidate: for (inv_id, inv, subs) in candidates {
            if subs.len() == 0 {
                successful_candidates.push(vec![inv_id]);
            }
            for frame in &self.justify_stack {
                if frame.base == inv && frame.subs == subs {
                    return Err(LookupInvariantError::DefinitelyDoesNotExist);
                }
            }
            if trace {
                let first = self.items.first().unwrap();
                let mut message = format!(
                    "\nAttempting to justify:\n    {}\nVia a theorem proving:\n    {}\nWith subs:",
                    indented(&self.show(statement, first)),
                    indented(&self.show(inv, first)),
                );
                for (target, value) in &subs {
                    message.push_str(&format!(
                        "\n{:?} ->\n    {}",
                        target,
                        indented(&self.show(*value, statement)),
                    ));
                }
                let bt = Backtrace::new();
                let depth = bt.frames().len();
                let indentation = format!("\n{}", vec![" "; depth].join(""));
                println!("{}", indented_with(&message, &indentation))
            }
            self.justify_stack.push(JustifyStackFrame {
                base: inv,
                subs: subs.clone(),
            });
            let mut justifications = Vec::new();
            let ok = self.check_subs(
                context,
                statement,
                subs,
                limit,
                &mut justifications,
                &mut err,
                trace,
            );
            self.justify_stack.pop();
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
        context: ItemId,
        statement: ItemId,
        subs: Substitutions,
        limit: u32,
        justifications: &mut Vec<InvariantSetId>,
        err: &mut LookupInvariantError,
        trace: bool,
    ) -> bool {
        let mut inv_subs = Substitutions::new();
        for (target, value) in subs {
            inv_subs.insert_no_replace(target, value);
            for invv in self.get_variable(target).clone().invariants {
                let statement = self.substitute_unchecked(invv, &inv_subs);
                let result = self.justify_statement(context, statement, limit - 1);
                match result {
                    Ok(new_justifications) => {
                        let set = self.push_invariant_set(InvariantSet::new_not_required(
                            statement,
                            vec![statement],
                            vec![statement],
                            hashset![],
                        ));
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
                            println!("{}", self.show(statement, statement));
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
