use std::collections::HashSet;

use super::Invariant;
use crate::{
    constructs::{substitution::Substitutions, Construct, GenInvResult},
    environment::{dependencies::DepResStackFrame, discover_equality::Equal, Environment, ItemId},
    scope::{LookupInvariantError, LookupInvariantResult, Scope},
};

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
            let mut adjusted_inv = inv;
            if !self.check_subs(subs, context, limit, &mut adjusted_inv, &mut err) {
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
