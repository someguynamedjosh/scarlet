pub mod justify;
mod tests;

use std::collections::HashSet;

use super::{dependencies::DepResStackFrame, discover_equality::Equal, Environment, ItemId};
use crate::{
    constructs::{substitution::Substitutions, Construct, GenInvResult},
    scope::{LookupInvariantError, LookupInvariantResult, Scope},
    shared::{Id, Pool},
};

pub type InvariantSetId = Id<'N'>;
pub type InvariantSetPool = Pool<InvariantSet, 'N'>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InvariantSet {
    statements: Vec<ItemId>,
    /// For the original statements to hold, all the statements in this list
    /// must also hold.
    pub(super) justification_requirements: Vec<ItemId>,
    pub(super) justified_by: Option<Vec<InvariantSetId>>,
    pub(super) dependencies: HashSet<ItemId>,
}

impl InvariantSet {
    pub fn new_empty() -> Self {
        Self::new(vec![], vec![])
    }

    pub fn new(statements: Vec<ItemId>, justification_requirements: Vec<ItemId>) -> Self {
        Self {
            statements,
            justification_requirements,
            justified_by: None,
            dependencies: HashSet::new(),
        }
    }

    pub(super) fn new_depending_on(dependencies: HashSet<ItemId>) -> InvariantSet {
        Self {
            statements: Vec::new(),
            justification_requirements: Vec::new(),
            justified_by: None,
            dependencies,
        }
    }

    /// Get a reference to the invariant set's statements.
    #[must_use]
    pub fn statements(&self) -> &[ItemId] {
        self.statements.as_ref()
    }

    /// Get a reference to the invariant set's justification requirements.
    #[must_use]
    pub fn justification_requirements(&self) -> &[ItemId] {
        self.justification_requirements.as_ref()
    }

    /// Get a reference to the invariant set's justified by.
    #[must_use]
    pub fn justified_by(&self) -> Option<&Vec<InvariantSetId>> {
        self.justified_by.as_ref()
    }

    /// Get a reference to the invariant set's dependencies.
    #[must_use]
    pub fn dependencies(&self) -> &HashSet<ItemId> {
        &self.dependencies
    }

    pub fn push(&mut self, statement: ItemId) {
        self.statements.push(statement);
    }
}

impl<'x> Environment<'x> {
    pub fn push_invariant_set(&mut self, invariant_set: InvariantSet) -> InvariantSetId {
        self.invariant_sets.push(invariant_set)
    }

    pub fn get_invariant_set(&self, invariant_set: InvariantSetId) -> &InvariantSet {
        &self.invariant_sets[invariant_set]
    }

    pub fn generated_invariants(&mut self, item_id: ItemId) -> InvariantSetId {
        for frame in &self.dep_res_stack {
            if frame.0 == item_id {
                return self.push_invariant_set(InvariantSet::new_empty());
            }
        }

        let result = if let Some(existing) = self.items[item_id].invariants {
            existing
        } else {
            self.dep_res_stack.push(DepResStackFrame(item_id));
            let context = match self.get_item_as_construct(item_id) {
                Ok(ok) => ok,
                Err(_err) => {
                    self.dep_res_stack.pop();
                    return self.push_invariant_set(InvariantSet::new_empty());
                }
            };
            let context = context.dyn_clone();
            let id = context.generated_invariants(item_id, self);
            self.items[item_id].invariants = Some(id);
            id
        };
        self.dep_res_stack.pop();
        result
    }

    pub fn get_produced_invariant(
        &mut self,
        statement: ItemId,
        context_id: ItemId,
        limit: u32,
    ) -> LookupInvariantResult {
        let generated_invariants = self.generated_invariants(context_id);
        for inv in self.invariant_sets[generated_invariants].clone().statements {
            if let Ok(equal) = self.discover_equal(inv, statement, limit) {
                if equal == Equal::yes() {
                    return Ok(generated_invariants);
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
