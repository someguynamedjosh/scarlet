use super::{
    as_builtin_value,
    base::{Construct, ConstructId},
    substitution::Substitutions,
    variable::CVariable,
};
use crate::{environment::Environment, impl_any_eq_for_construct, shared::TripleBool};

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

    fn check<'x>(&self, _env: &mut Environment<'x>) {}

    fn get_dependencies<'x>(&self, _env: &mut Environment<'x>) -> Vec<CVariable> {
        Vec::new()
    }

    fn is_def_equal<'x>(&self, env: &mut Environment<'x>, other: &dyn Construct) -> TripleBool {
        TripleBool::Unknown
    }

    fn substitute<'x>(
        &self,
        env: &mut Environment<'x>,
        _substitutions: &Substitutions,
    ) -> ConstructId {
        env.push_construct(Box::new(self.clone()))
    }
}
