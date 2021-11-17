use super::base::{Construct, ConstructId};
use crate::impl_any_eq_for_construct;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Member {
    Named(String),
    Index {
        index: ConstructId,
        proof_lt_len: ConstructId,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CMember(pub ConstructId, pub Member);

impl_any_eq_for_construct!(CMember);

impl Construct for CMember {
    fn dyn_clone(&self) -> Box<dyn Construct> {
        Box::new(self.clone())
    }
}
