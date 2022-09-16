use super::{Icc, PredicateSet, OnlyCalledByIcc};
use crate::item::{resolvable::UnresolvedItemError, ItemPtr};

pub type PredicatesResult = Result<PredicateSet, UnresolvedItemError>;

pub trait PredicatesFeature {
    #[allow(unused_variables)]
    fn get_predicates_using_context(
        &self,
        this: &ItemPtr,
        ctx: &mut Icc,
        _: OnlyCalledByIcc,
    ) -> PredicatesResult {
        Ok(PredicateSet::new_empty())
    }
}
