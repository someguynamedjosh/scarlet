use std::collections::HashSet;

use backtrace::Backtrace;

use super::Invariant;
use crate::{
    constructs::{substitution::Substitutions, Construct, GenInvResult},
    environment::{dependencies::DepResStackFrame, discover_equality::Equal, Environment, ItemId},
    scope::{LookupInvariantError, LookupInvariantResult, Scope},
    shared::{indented, indented_with},
};

#[derive(Clone, Debug)]
pub struct JustifyStackFrame {
    base: ItemId,
    subs: Substitutions,
}

pub type JustifyStack = Vec<JustifyStackFrame>;

impl<'x> Environment<'x> {
    pub fn justify(
        &mut self,
        statement: ItemId,
        context: ItemId,
        limit: u32,
    ) -> LookupInvariantResult {
        match self.get_produced_invariant(statement, context, limit) {
            Ok(inv) => Ok(inv),
            Err(err) => self.create_justification(statement, context, limit, err),
        }
    }

    fn create_justification(
        &mut self,
        statement: ItemId,
        context: ItemId,
        limit: u32,
        mut err: LookupInvariantError,
    ) -> LookupInvariantResult {
        let trace = true;
        if limit == 0 {
            return Err(err);
        }
        let mut candidates = Vec::new();
        for at in self.auto_theorems.clone() {
            for inv in self.generated_invariants(at) {
                match self.discover_equal(inv.statement, statement, limit - 1)? {
                    Equal::Yes(subs) => candidates.push((inv, subs)),
                    Equal::NeedsHigherLimit => err = LookupInvariantError::MightNotExist,
                    _ => (),
                }
            }
        }
        'check_next_candidate: for (inv, subs) in candidates {
            if subs.len() == 0 {
                return Ok(inv);
            }
            for frame in &self.justify_stack {
                if frame.base == inv.statement && frame.subs == subs {
                    return Err(LookupInvariantError::DefinitelyDoesNotExist);
                }
            }
            if trace {
                let mut message = format!(
                    "\nAttempting to justify:\n{}\nWith subs:\n{:#?}",
                    self.show(inv.statement, context)
                        .unwrap_or("unresolved".to_owned()),
                    subs
                );
                for (target, value) in &subs {
                    message.push_str(&format!(
                        "\n{:?} ->\n{}",
                        target,
                        self.show(*value, context)
                            .unwrap_or("unresolved".to_owned())
                    ));
                }
                let bt = Backtrace::new();
                let depth = bt.frames().len();
                let indentation = format!("\n{}", vec![" "; depth].join(""));
                println!("{}", indented_with(&message, &indentation))
            }
            self.justify_stack.push(JustifyStackFrame {
                base: inv.statement,
                subs: subs.clone(),
            });
            let mut adjusted_inv = inv;
            let ok = self.check_subs(subs, context, limit, &mut adjusted_inv, &mut err);
            self.justify_stack.pop();
            if !ok {
                continue 'check_next_candidate;
            }
            return Ok(adjusted_inv);
        }
        Err(err)
    }

    fn check_subs(
        &mut self,
        subs: Substitutions,
        context: ItemId,
        limit: u32,
        adjusted_inv: &mut Invariant,
        err: &mut LookupInvariantError,
    ) -> bool {
        let mut inv_subs = Substitutions::new();
        for (target, value) in subs {
            inv_subs.insert_no_replace(target, value);
            for invv in self.get_variable(target).clone().invariants {
                let statement = self.substitute(invv, &inv_subs);
                let result = self.justify(statement, context, limit - 1);
                match result {
                    Ok(inv) => {
                        for dep in inv.dependencies {
                            adjusted_inv.dependencies.insert(dep);
                        }
                    }
                    Err(LookupInvariantError::Unresolved(..))
                    | Err(LookupInvariantError::MightNotExist) => {
                        *err = result.unwrap_err();
                        return false;
                    }
                    Err(LookupInvariantError::DefinitelyDoesNotExist) => {
                        return false;
                    }
                }
            }
        }
        true
    }
}
