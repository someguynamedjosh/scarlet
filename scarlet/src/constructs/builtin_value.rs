use super::base::{Construct, ConstructId};
use crate::{environment::Environment, impl_any_eq_for_construct};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CBuiltinValue {
    Bool(bool),
    _32U(u32)
}

impl_any_eq_for_construct!(CBuiltinValue);

impl Construct for CBuiltinValue {
    fn dyn_clone(&self) -> Box<dyn Construct> {
        Box::new(self.clone())
    }
}
