use super::EqualResult;
use crate::item::ItemPtr;

pub struct EqualityCalculationContext {}

pub type Ecc = EqualityCalculationContext;

/// Using this in a function signature guarantees that only
/// EqualityCalculationContext can call that function. If you are reusing this
/// inside the function that is being called, you are doing something wrong.
pub struct OnlyCalledByEcc(());

/// Used to restrict calls to `Ecc::refine_and_get_equality()`.
#[derive(Clone, Copy)]
pub struct PermissionToRefine(());

impl EqualityCalculationContext {
    pub fn rhs(&self) -> &ItemPtr {
        todo!()
    }

    pub fn get_equality(lhs: ItemPtr, rhs: ItemPtr, limit: u32) -> EqualResult {
        todo!()
    }

    pub fn refine_and_get_equality(
        &self,
        new_lhs: ItemPtr,
        new_rhs: ItemPtr,
        _: PermissionToRefine,
    ) -> EqualResult {
        todo!()
    }

    fn get_equality_impl(&self) -> EqualResult {
        todo!()
    }
}
