use super::base::{Construct, ConstructId};
use crate::{environment::Environment, impl_any_eq_for_construct};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CBuiltinValue {
    Bool(bool),
    _32U(u32),
}

impl CBuiltinValue {
    pub fn as_bool(&self) -> Option<bool> {
        if let Self::Bool(v) = self {
            Some(*v)
        } else {
            None
        }
    }

    pub fn as_32u(&self) -> Option<u32> {
        if let Self::_32U(v) = self {
            Some(*v)
        } else {
            None
        }
    }
}

impl_any_eq_for_construct!(CBuiltinValue);

impl Construct for CBuiltinValue {
    fn dyn_clone(&self) -> Box<dyn Construct> {
        Box::new(self.clone())
    }
}
