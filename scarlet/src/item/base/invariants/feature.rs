use super::{Icc, InvariantSet, InvariantSetPtr, OnlyCalledByIcc};
use crate::item::{resolvable::UnresolvedItemError, ItemPtr};

pub type InvariantsResult = Result<InvariantSetPtr, UnresolvedItemError>;

pub trait InvariantsFeature {
    #[allow(unused_variables)]
    fn get_invariants_using_context(
        &self,
        this: &ItemPtr,
        ctx: &mut Icc,
        _: OnlyCalledByIcc,
    ) -> InvariantsResult {
        Ok(InvariantSet::new_empty(this.ptr_clone()))
    }
}
