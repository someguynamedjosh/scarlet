pub mod justify;
mod tests;

use std::collections::HashSet;

use super::{dependencies::DepResStackFrame, discover_equality::Equal, Environment, ItemId};
use crate::{
    constructs::{substitution::Substitutions, Construct, GenInvResult},
    scope::{LookupInvariantError, LookupInvariantResult, Scope},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Invariant {
    pub statement: ItemId,
    pub dependencies: HashSet<ItemId>,
}

impl Invariant {
    pub fn new(statement: ItemId, dependencies: HashSet<ItemId>) -> Self {
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
        if let Equal::Yes(subs) = incoming.1 {
            let better_than_best_match = self.0.as_ref().map(|(_, bsubs)| bsubs.len() > subs.len());
            if better_than_best_match.unwrap_or(true) {
                self.0 = Some((incoming.0, subs));
            }
        }
    }

    pub fn pack(self) -> Result<(Invariant, Substitutions), ()> {
        self.0.ok_or(())
    }
}

impl<'x> Environment<'x> {
    pub fn generated_invariants(&mut self, item_id: ItemId) -> GenInvResult {
        for frame in &self.dep_res_stack {
            if frame.0 == item_id {
                return Vec::new();
            }
        }

        self.dep_res_stack.push(DepResStackFrame(item_id));
        let context = match self.get_item_as_construct(item_id) {
            Ok(ok) => ok,
            Err(_err) => {
                self.dep_res_stack.pop();
                return Vec::new();
            }
        };
        let context = context.dyn_clone();
        let invs = context.generated_invariants(item_id, self);
        self.items[item_id].invariants = Some(invs.clone());
        self.dep_res_stack.pop();
        invs
    }

    pub fn get_produced_invariant(
        &mut self,
        statement: ItemId,
        context_id: ItemId,
        limit: u32,
    ) -> LookupInvariantResult {
        let generated_invariants = self.generated_invariants(context_id);
        for inv in generated_invariants {
            if let Ok(equal) = self.discover_equal(inv.statement, statement, limit) {
                if equal == Equal::yes() {
                    return Ok(inv);
                }
            }
        }
        let scope = self.get_item(context_id).scope.dyn_clone();
        scope.lookup_invariant_limited(self, statement, limit)
    }

    pub fn add_auto_theorem(&mut self, auto_theorem: ItemId) {
        self.auto_theorems.push(auto_theorem);
    }
}
