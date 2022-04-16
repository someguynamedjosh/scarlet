use super::Icc;
use crate::{
    environment::invariants::{InvariantSet, InvariantSetPtr},
    item::{resolvable::UnresolvedItemError, ItemPtr},
};

pub type InvariantsResult = Result<InvariantSetPtr, UnresolvedItemError>;

pub trait InvariantsFeature {
    #[allow(unused_variables)]
    fn get_invariants_using_context(&self, this: &ItemPtr, ctx: &mut Icc) -> InvariantsResult {
        InvariantSet::new_empty(this)
    }
}
