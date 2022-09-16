use super::{feature::PredicatesResult, PredicateSet};
use crate::item::{base::util::RecursionPreventionStack, ItemPtr};

/// Using this in a function signature guarantees that only
/// InvariantCalculationContext can call that function. If you are reusing this
/// inside the function that is being called, you are doing something wrong.
pub struct OnlyCalledByIcc(());

pub struct InvariantCalculationContext {
    stack: RecursionPreventionStack,
}

pub type Icc = InvariantCalculationContext;

impl InvariantCalculationContext {
    pub fn get_invariants(&mut self, of_item: &ItemPtr) -> PredicatesResult {
        let of_item = of_item.dereference();
        RecursionPreventionStack::skip_recursion_or_execute_with_mutable_access(
            self,
            &of_item,
            |s| &mut s.stack,
            |this| {
                let def = &of_item.borrow().definition;
                def.get_predicates_using_context(&of_item, this, OnlyCalledByIcc(()))
            },
        )
        .unwrap_or_else(|| Ok(PredicateSet::new_empty()))
    }

    pub fn new() -> Self {
        Self {
            stack: RecursionPreventionStack::new(),
        }
    }
}
