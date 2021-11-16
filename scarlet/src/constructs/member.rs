use super::base::{Construct, ConstructId};
use crate::impl_any_eq_for_construct;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Member {
    Named(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CMember(pub ConstructId, pub Member);

impl_any_eq_for_construct!(CMember);

impl Construct for CMember {}
