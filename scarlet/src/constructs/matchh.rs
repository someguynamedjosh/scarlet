use itertools::Itertools;

use super::{
    base::{Construct, ConstructId},
    substitution::Substitutions,
    variable::CVariable,
};
use crate::{environment::Environment, impl_any_eq_for_construct, shared::TripleBool};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Condition {
    pub pattern: ConstructId,
    pub value: ConstructId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CMatch {
    pub base: ConstructId,
    pub conditions: Vec<Condition>,
    pub else_value: ConstructId,
}

impl_any_eq_for_construct!(CMatch);

impl Construct for CMatch {
    fn dyn_clone(&self) -> Box<dyn Construct> {
        Box::new(self.clone())
    }

    fn check<'x>(&self, _env: &mut Environment<'x>) {}

    fn get_dependencies<'x>(&self, env: &mut Environment<'x>) -> Vec<CVariable> {
        let mut deps = Vec::new();
        deps.append(&mut env.get_dependencies(self.base));
        deps.append(&mut env.get_dependencies(self.else_value));
        for con in &self.conditions {
            deps.append(&mut env.get_non_capturing_dependencies(con.pattern));
            deps.append(&mut env.get_dependencies(con.value))
        }
        deps
    }

    fn is_def_equal<'x>(&self, env: &mut Environment<'x>, other: &dyn Construct) -> TripleBool {
        TripleBool::Unknown
    }

    fn substitute<'x>(
        &self,
        env: &mut Environment<'x>,
        substitutions: &Substitutions,
    ) -> ConstructId {
        let base = env.substitute(self.base, substitutions);
        let else_value = env.substitute(self.else_value, substitutions);
        let conditions = self
            .conditions
            .iter()
            .map(|con| Condition {
                pattern: env.substitute(con.pattern, substitutions),
                value: env.substitute(con.value, substitutions),
            })
            .collect_vec();
        env.push_construct(Box::new(Self {
            base,
            conditions,
            else_value,
        }))
    }
}
