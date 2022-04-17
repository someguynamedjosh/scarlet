use super::{Ecc, Equal};
use crate::item::resolvable::UnresolvedItemError;

pub type EqualResult = Result<Equal, UnresolvedItemError>;

pub trait EqualityFeature {
    #[allow(unused_variables)]
    fn get_equality_using_context(&self, ctx: &Ecc) -> EqualResult {
        Ok(Equal::Unknown)
    }
}
