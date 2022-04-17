use super::{DepResult, Dependencies};
use crate::item::{base::util::RecursionPreventionStack, ItemPtr};

/// Using this in a function signature guarantees that only
/// DependencyCalculationContext can call that function. If you are reusing this
/// inside the function that is being called, you are doing something wrong.
pub struct OnlyCalledByDcc(());

pub struct DependencyCalculationContext {
    stack: RecursionPreventionStack,
}

pub type Dcc = DependencyCalculationContext;

impl DependencyCalculationContext {
    pub fn get_dependencies(&mut self, of_item: &ItemPtr) -> DepResult {
        self.stack
            .skip_recursion_or_execute(of_item, || {
                let def = of_item.borrow().definition;
                let mut deps = def.get_dependencies_using_context(self, OnlyCalledByDcc(()));
                deps.skipped_due_to_recursion.remove(of_item);
                deps
            })
            .unwrap_or_else(|| Dependencies::new_missing(of_item.ptr_clone()))
    }

    pub fn new() -> Self {
        Self {
            stack: RecursionPreventionStack::new(),
        }
    }
}
