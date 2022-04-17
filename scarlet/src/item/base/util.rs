use crate::item::{
    definitions::substitution::{DSubstitution, Substitutions},
    Item, ItemPtr,
};

pub(super) struct RecursionPreventionStack(Vec<ItemPtr>);

impl RecursionPreventionStack {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Executes the given function if the given item is not on the stack.
    /// Pushes the item onto the stack for the duration of the function.
    pub fn skip_recursion_or_execute<T>(
        &mut self,
        key: &ItemPtr,
        func: impl FnOnce() -> T,
    ) -> Option<T> {
        if self
            .0
            .iter()
            .any(|previous_key| previous_key.is_same_instance_as(key))
        {
            None
        } else {
            self.0.push(key.ptr_clone());
            let result = func();
            assert!(self.0.pop().unwrap().is_same_instance_as(key));
            Some(result)
        }
    }
}
