use super::{
    as_builtin_value,
    base::{Construct, ConstructId},
    substitution::Substitutions,
    variable::{CVariable, VarType},
};
use crate::{
    environment::{matchh::MatchResult, Environment},
    impl_any_eq_for_construct,
    tokens::structure::Token,
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

    fn check<'x>(&self, _env: &mut Environment<'x>) {}

    fn get_dependencies<'x>(&self, env: &mut Environment<'x>) -> Vec<CVariable> {
        Vec::new()
    }

    fn matches_simple_var_type<'x>(
        &self,
        env: &mut Environment<'x>,
        pattern: &VarType,
    ) -> MatchResult {
        match (self, pattern) {
            (CBuiltinValue::Bool(_), VarType::Bool) | (CBuiltinValue::_32U(_), VarType::_32U) => {
                MatchResult::non_capturing()
            }
            (_, VarType::Just(pattern)) => {
                if let Some(value) = as_builtin_value(&**env.get_construct(*pattern)) {
                    if self == value {
                        MatchResult::non_capturing()
                    } else {
                        MatchResult::NoMatch
                    }
                } else {
                    MatchResult::Unknown
                }
            }
            _ => MatchResult::NoMatch,
        }
    }

    fn substitute<'x>(
        &self,
        env: &mut Environment<'x>,
        _substitutions: &Substitutions,
    ) -> ConstructId {
        env.push_construct(Box::new(self.clone()))
    }
}
