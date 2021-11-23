use super::{
    base::{Construct, ConstructId},
    substitution::Substitutions,
    variable::{CVariable, VarType},
};
use crate::{
    constructs::{as_struct, builtin_value::CBuiltinValue},
    environment::{matchh::MatchResult, Environment},
    impl_any_eq_for_construct,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CLength(pub ConstructId);

impl_any_eq_for_construct!(CLength);

impl Construct for CLength {
    fn dyn_clone(&self) -> Box<dyn Construct> {
        Box::new(self.clone())
    }

    fn check<'x>(&self, _env: &mut Environment<'x>) {}

    fn get_dependencies<'x>(&self, env: &mut Environment<'x>) -> Vec<CVariable> {
        env.get_dependencies(self.0)
    }

    fn matches_simple_var_type<'x>(
        &self,
        _env: &mut Environment<'x>,
        pattern: &VarType,
    ) -> MatchResult {
        match pattern {
            VarType::Bool => MatchResult::NoMatch,
            VarType::_32U => MatchResult::non_capturing(),
            _ => MatchResult::Unknown,
        }
    }

    fn reduce<'x>(&self, env: &mut Environment<'x>, _self_id: ConstructId) -> ConstructId {
        let base = env.reduce(self.0);
        let base_con = env.get_construct(base);
        if let Some(structt) = as_struct(&**base_con) {
            let length = structt.0.len() as u32;
            env.push_construct(Box::new(CBuiltinValue::_32U(length)))
        } else {
            env.push_construct(Box::new(Self(base)))
        }
    }

    fn substitute<'x>(
        &self,
        env: &mut Environment<'x>,
        substitutions: &Substitutions,
    ) -> ConstructId {
        let base = env.substitute(self.0, substitutions);
        env.push_construct(Box::new(Self(base)))
    }
}
