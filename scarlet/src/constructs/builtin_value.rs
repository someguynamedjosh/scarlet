use super::{
    base::{Construct, ConstructId},
    variable::VarType,
};
use crate::{
    environment::{matchh::MatchResult, Environment},
    impl_any_eq_for_construct,
};

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

    fn matches_var_type<'x>(&self, _env: &mut Environment<'x>, pattern: &VarType) -> MatchResult {
        match (self, pattern) {
            (CBuiltinValue::Bool(_), VarType::Bool) | (CBuiltinValue::_32U(_), VarType::_32U) => {
                MatchResult::non_capturing()
            }
            _ => MatchResult::Unknown,
        }
    }
}
