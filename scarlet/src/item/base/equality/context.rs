use std::cell::Ref;

use owning_ref::OwningRef;

use super::{item_wrapper::ItemWithSubsAndRecursion, Equal, EqualResult};
use crate::item::{
    definitions::{
        other::DOther,
        substitution::{DSubstitution, Substitutions},
    },
    Item, ItemPtr,
};

pub struct EqualityCalculationContext {
    pub(super) lhs: ItemWithSubsAndRecursion,
    pub(super) rhs: ItemWithSubsAndRecursion,
    pub(super) limit: u32,
}

pub type Ecc = EqualityCalculationContext;

/// Using this in a function signature guarantees that only
/// EqualityCalculationContext can call that function. If you are reusing this
/// inside the function that is being called, you are doing something wrong.
pub struct OnlyCalledByEcc(pub(super) ());

/// Used to restrict calls to `Ecc::refine_and_get_equality()`.
#[derive(Clone, Copy)]
pub struct PermissionToRefine(pub(super) ());

impl EqualityCalculationContext {
    pub fn rhs(&self) -> &ItemPtr {
        &self.rhs.item
    }

    pub fn get_equality(lhs: ItemPtr, rhs: ItemPtr, limit: u32) -> EqualResult {
        Self {
            lhs: ItemWithSubsAndRecursion {
                item: lhs,
                subs: vec![],
                recurses_over: vec![],
            },
            rhs: ItemWithSubsAndRecursion {
                item: rhs,
                subs: vec![],
                recurses_over: vec![],
            },
            limit,
        }
        .get_equality_impl()
    }

    pub fn refine_and_get_equality(
        &mut self,
        mut new_lhs: ItemPtr,
        mut new_rhs: ItemPtr,
        _: PermissionToRefine,
    ) -> EqualResult {
        std::mem::swap(&mut new_lhs, &mut self.lhs.item);
        std::mem::swap(&mut new_rhs, &mut self.rhs.item);
        self.limit -= 1;
        let result = self.get_equality_impl();
        // Put the old ones back.
        std::mem::swap(&mut new_lhs, &mut self.lhs.item);
        std::mem::swap(&mut new_rhs, &mut self.rhs.item);
        self.limit += 1;
        result
    }
}
