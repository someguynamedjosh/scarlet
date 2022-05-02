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
        result.push(item.get_invariants().unwrap());
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
        const MAX_LIMIT: u32 = 16;
        for limit in 0..MAX_LIMIT {
            println!("{}/{}", limit, MAX_LIMIT);
            for set_ptr in self.sets.clone() {
                let set = set_ptr.borrow();
                if set.connected_to_root {
                    continue;
                }
                drop(set);
                let res = self.justify_set(&set_ptr, limit);
                let set = set_ptr.borrow();
                if limit == MAX_LIMIT - 1 && !set.connected_to_root {
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

    fn justify_set(
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
        println!("----------------------------------------");
        println!(
            "{}",
            self.env.show(statement.ptr_clone(), context.ptr_clone())
        );
        let mut result = Vec::new();
        let ctx_scope = context.clone_scope();
        let available_invariant_sets = ctx_scope.get_invariant_sets();
        let iterate_over = available_invariant_sets;
        for other_set in iterate_over {
            for other_statement in other_set.borrow().statements() {
                if TRACE || true {
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
        if result.len() == 0 {
            Err(LookupInvariantError::MightNotExist)
        } else {
            Ok(result)
        }
    }
}
