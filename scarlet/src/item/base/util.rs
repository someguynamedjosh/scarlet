use itertools::Itertools;

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

    pub fn skip_recursion_or_execute_with_mutable_access<B, T>(
        base: &mut B,
        key: &ItemPtr,
        get_self: impl Fn(&mut B) -> &mut Self,
        func: impl FnOnce(&mut B) -> T,
    ) -> Option<T> {
        let this = get_self(base);
        if this
            .0
            .iter()
            .any(|previous_key| previous_key.is_same_instance_as(key))
        {
            None
        } else {
            this.0.push(key.ptr_clone());
            let result = func(base);
            let this = get_self(base);
            assert!(this.0.pop().unwrap().is_same_instance_as(key));
            Some(result)
        }
    }
}

#[derive(Clone)]
pub(super) struct Stack<T>(Vec<T>);

impl<T> Stack<T>
where
    T: PartialEq,
{
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn contains(&self, t: &T) -> bool {
        self.0.contains(t)
    }

    pub fn with_stack_frame<R>(&mut self, key: T, func: impl FnOnce(&mut Self) -> R) -> R {
        let len_before_executing = self.0.len();
        self.0.push(key);
        let result = func(self);
        self.0.pop();
        assert_eq!(self.0.len(), len_before_executing);
        result
    }

    pub fn into_frames(self) -> Vec<T> {
        self.0
    }

    pub(crate) fn frames(&self) -> &[T] {
        &self.0
    }
}
