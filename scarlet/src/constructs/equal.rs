use itertools::Itertools;

use super::{
    substitution::Substitutions, variable::CVariable, Construct, ConstructDefinition, ConstructId,
};
use crate::{
    environment::Environment, impl_any_eq_for_construct, scope::SPlain, shared::TripleBool,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CEqual(ConstructId, ConstructId);

impl CEqual {
    pub fn new<'x>(
        env: &mut Environment<'x>,
        left: ConstructId,
        right: ConstructId,
    ) -> ConstructId {
        let con = env.push_construct(Self(left, right));
        env.set_scope(left, &SPlain(con));
        env.set_scope(right, &SPlain(con));
        con
    }
}

impl_any_eq_for_construct!(CEqual);

impl Construct for CEqual {
    fn dyn_clone(&self) -> Box<dyn Construct> {
        Box::new(self.clone())
    }

    fn check<'x>(&self, _env: &mut Environment<'x>) {}

    fn get_dependencies<'x>(&self, env: &mut Environment<'x>) -> Vec<CVariable> {
        [env.get_dependencies(self.0), env.get_dependencies(self.1)].concat()
    }

    fn is_def_equal<'x>(&self, _env: &mut Environment<'x>, _other: &dyn Construct) -> TripleBool {
        TripleBool::Unknown
    }

    fn reduce<'x>(&self, env: &mut Environment<'x>) -> ConstructDefinition<'x> {
        match env.is_def_equal(self.0, self.1) {
            TripleBool::True => env.get_builtin_item("true").into(),
            TripleBool::False => env.get_builtin_item("false").into(),
            TripleBool::Unknown => self.dyn_clone().into(),
        }
    }

    fn substitute<'x>(
        &self,
        env: &mut Environment<'x>,
        substitutions: &Substitutions,
    ) -> ConstructId {
        let left = env.substitute(self.0, substitutions);
        let right = env.substitute(self.1, substitutions);
        Self::new(env, left, right)
    }
}
