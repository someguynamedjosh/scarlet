use std::cell::Ref;

use owning_ref::OwningRef;

use super::EqualResult;
use crate::item::{
    definitions::{
        other::DOther,
        substitution::{DSubstitution, Substitutions}, variable::VariablePtr,
    },
    Item, ItemPtr,
};

#[derive(Clone)]
pub(super) struct ItemWithSubsAndRecursion {
    pub(super) item: ItemPtr,
    /// Each set of substitutions is applied in the order they were inserted.
    pub(super) subs: Vec<Substitutions>,
    pub(super) recurses_over: Vec<ItemPtr>,
}

impl ItemWithSubsAndRecursion {
    /// Returns true if self was modified.
    pub fn dereference_once(&mut self) -> bool {
        let new_item = if let Some(other) = self.item.downcast_definition::<DOther>() {
            if other.is_recursive() {
                self.recurses_over.push(other.other().ptr_clone());
            }
            Some(other.other().ptr_clone())
        } else if let Some(dereffed) = self.item.dereference_once() {
            Some(dereffed)
        } else if let Some(sub) = self.item.downcast_definition::<DSubstitution>() {
            let subs = sub.substitutions();
            let mut filtered = Substitutions::new();
            for dep in sub.base().get_dependencies().into_variables() {
                if let Some(rep) = subs.get(&dep.var) {
                    filtered.insert_no_replace(dep.var, rep.ptr_clone());
                }
            }
            self.subs.push(filtered);
            Some(sub.base().ptr_clone())
        } else {
            None
        };
        if let Some(item) = new_item {
            self.item = item;
            true
        } else {
            false
        }
    }

    /// Modifies self assuming the base has been substituted with value.
    pub fn select_substitution(&mut self, index: usize, target: &VariablePtr, value: ItemPtr) {
        self.subs[index].remove(target);
        self.item = value;
    }
}
