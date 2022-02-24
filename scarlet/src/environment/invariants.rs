mod tests;

use std::collections::HashSet;

use super::{dependencies::DepResStackFrame, discover_equality::Equal, ConstructId, Environment};
use crate::{
    constructs::{substitution::Substitutions, Construct, GenInvResult},
    scope::{LookupInvariantError, LookupInvariantResult, LookupSimilarInvariantResult, Scope},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Invariant {
    pub statement: ConstructId,
    pub dependencies: HashSet<ConstructId>,
}

impl Invariant {
    pub fn new(statement: ConstructId, dependencies: HashSet<ConstructId>) -> Self {
        Self {
            statement,
            dependencies,
        }
    }
}

pub struct InvariantMatch(Option<(Invariant, Substitutions)>);

impl InvariantMatch {
    pub fn new() -> Self {
        Self(None)
    }

    pub fn switch_if_better(&mut self, incoming: (Invariant, Equal)) {
        if let Equal::Yes(l) = incoming.1 {
            let better_than_best_match = self.0.as_ref().map(|(_, bl)| bl.len() > l.len());
            if better_than_best_match.unwrap_or(true) {
                self.0 = Some((incoming.0, l));
            }
        }
    }

    pub fn pack(self) -> Result<(Invariant, Equal), ()> {
        if let Some((inv, l)) = self.0 {
            Ok((inv, Equal::Yes(l)))
        } else {
            Err(())
        }
    }

    pub fn switch_if_better_then_pack(
        mut self,
        incoming: LookupSimilarInvariantResult,
    ) -> LookupSimilarInvariantResult {
        match incoming {
            Ok(incoming) => {
                self.switch_if_better(incoming);
                self.pack()
                    .map_err(|_| LookupInvariantError::DefinitelyDoesNotExist)
            }
            Err(err) => self.pack().map_err(|_| err),
        }
    }
}

impl<'x> Environment<'x> {
    pub fn generated_invariants(&mut self, con_id: ConstructId) -> GenInvResult {
        for frame in &self.dep_res_stack {
            if frame.0 == con_id {
                return Vec::new();
            }
        }

        self.dep_res_stack.push(DepResStackFrame(con_id));
        let context = match self.get_construct_definition(con_id) {
            Ok(ok) => ok,
            Err(_err) => {
                self.dep_res_stack.pop();
                return Vec::new();
            }
        };
        let context = context.dyn_clone();
        let invs = context.generated_invariants(con_id, self);
        self.constructs[con_id].invariants = Some(invs.clone());
        self.dep_res_stack.pop();
        invs
    }

    pub fn get_produced_invariant(
        &mut self,
        statement: ConstructId,
        context_id: ConstructId,
        limit: u32,
    ) -> LookupSimilarInvariantResult {
        let generated_invariants = self.generated_invariants(context_id);
        let mut best_match = InvariantMatch::new();
        let mut default_error = LookupInvariantError::DefinitelyDoesNotExist;
        for inv in generated_invariants {
            if let Ok(equal) = self.discover_equal(inv.statement, statement, limit) {
                if equal.is_needs_higher_limit() {
                    default_error = LookupInvariantError::MightNotExist;
                }
                best_match.switch_if_better((inv, equal));
            }
        }
        let scope = self.get_construct(context_id).scope.dyn_clone();
        let other_contender = scope.lookup_invariant_limited(self, statement, limit);
        match best_match.switch_if_better_then_pack(other_contender) {
            Ok(ok) => Ok(ok),
            Err(err) => Err(if err == LookupInvariantError::DefinitelyDoesNotExist {
                default_error
            } else {
                err
            }),
        }
    }

    pub fn justify(
        &mut self,
        statement: ConstructId,
        context: ConstructId,
        limit: u32,
    ) -> LookupInvariantResult {
        let root = self.get_produced_invariant(statement, context, limit)?;
        if let Equal::Yes(l) = root.1 {
            if l.len() > 0 {
                todo!()
            } else {
                Ok(root.0)
            }
        } else {
            unreachable!()
        }
    }
}
