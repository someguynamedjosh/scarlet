use std::collections::HashSet;

use backtrace::Backtrace;
use maplit::hashset;

use super::{InvariantSet, InvariantSetId};
use crate::{
    constructs::{substitution::Substitutions, Construct, GenInvResult},
    environment::{dependencies::DepResStackFrame, discover_equality::Equal, Environment, ItemId},
    scope::{LookupInvariantError, LookupInvariantResult, Scope},
    shared::{indented, indented_with},
};

pub type JustifyInvariantResult = Result<Vec<InvariantSetId>, LookupInvariantError>;

#[derive(Clone, Debug)]
pub struct JustifyStackFrame {
    base: ItemId,
    subs: Substitutions,
}

pub type JustifyStack = Vec<JustifyStackFrame>;

impl<'x> Environment<'x> {
    pub(super) fn justify(&mut self, set: InvariantSetId, limit: u32) {
        let set = self.invariant_sets[set].clone();
        for (other_id, other_set) in &self.invariant_sets {
            todo!()
        }
        todo!()
    }

    pub(super) fn justify_once(&mut self, statement: ItemId, limit: u32) -> JustifyInvariantResult {
        for (other_id, other_set) in &self.invariant_sets {
            todo!()
        }
        todo!()
    }

    fn create_justification(
        &mut self,
        statement: ItemId,
        context: ItemId,
        limit: u32,
        mut err: LookupInvariantError,
    ) -> JustifyInvariantResult {
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
                let inv = self
                    .push_invariant_set(InvariantSet::new_depending_on(rec.into_iter().collect()));
                return Ok(vec![inv]);
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
        'check_next_candidate: for (inv_id, inv, subs) in candidates {
            if subs.len() == 0 {
                return Ok(vec![inv_id]);
            }
            for frame in &self.justify_stack {
                if frame.base == inv && frame.subs == subs {
                    return Err(LookupInvariantError::DefinitelyDoesNotExist);
                }
            }
            if trace {
                let mut message = format!(
                    "\nAttempting to justify:\n    {}\nVia a theorem proving:\n    {}\nWith subs:",
                    indented(&self.show(statement, context)),
                    indented(&self.show(inv, context)),
                );
                for (target, value) in &subs {
                    message.push_str(&format!(
                        "\n{:?} ->\n    {}",
                        target,
                        indented(&self.show(*value, context)),
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
            let ok = self.check_subs(subs, context, limit, &mut justifications, &mut err, trace);
            self.justify_stack.pop();
            if !ok {
                continue 'check_next_candidate;
            }
            return Ok(justifications);
        }
        Err(err)
    }

    fn check_subs(
        &mut self,
        subs: Substitutions,
        context: ItemId,
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
                let result = self.justify_once(statement, limit - 1);
                match result {
                    Ok(mut new_justifications) => {
                        justifications.append(&mut new_justifications);
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
                            println!("{}", self.show(statement, context));
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
