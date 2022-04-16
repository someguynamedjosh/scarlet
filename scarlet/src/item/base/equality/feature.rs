use super::{Ecc, Equal};
use crate::item::resolvable::UnresolvedItemError;

pub type EqualResult = Result<Equal, UnresolvedItemError>;

pub trait EqualityFeature {
    #[allow(unused_variables)]
    fn discover_equality(&self, ctx: &Ecc) -> EqualResult {
        Ok(Equal::Unknown)
    }
}
