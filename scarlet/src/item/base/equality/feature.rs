use super::{Ecc, Equal, OnlyCalledByEcc};
use crate::item::resolvable::UnresolvedItemError;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EqualSuccess {
    pub equal: Equal,
    pub unique: bool,
}

pub type EqualResult = Result<EqualSuccess, UnresolvedItemError>;

pub trait EqualityFeature {
    #[allow(unused_variables)]
    fn get_equality_using_context(&self, ctx: &mut Ecc, _: OnlyCalledByEcc) -> EqualResult {
        Ok(EqualSuccess {
            equal: Equal::Unknown,
            unique: true,
        })
    }
}
