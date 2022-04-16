use super::feature::InvariantsResult;
use crate::{
    environment::invariants::InvariantSet,
    item::{base::util::RecursionPreventionStack, ItemPtr, ItemPtrExtensions},
};

/// Using this in a function signature guarantees that only
/// InvariantCalculationContext can call that function.
pub struct OnlyCalledByIcc(());

pub struct InvariantCalculationContext {
    stack: RecursionPreventionStack,
}

pub type Icc = InvariantCalculationContext;

impl InvariantCalculationContext {
    pub fn get_dependencies(&mut self, of_item: &ItemPtr) -> InvariantsResult {
        self.stack
            .skip_recursion_or_execute(of_item, || {
                let def = of_item.borrow().definition;
                let mut invs = def.get_invariants_using_context(self, OnlyCalledByIcc(()));
                invs
            })
            .unwrap_or_else(|| Ok(InvariantSet::new_empty(of_item.ptr_clone())))
    }

    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }
}
