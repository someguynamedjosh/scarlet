use itertools::Itertools;

use super::{
    base::{Construct, ConstructId},
    substitution::Substitutions,
    variable::{CVariable, VarType},
};
use crate::{
    environment::{matchh::MatchResult, Environment},
    impl_any_eq_for_construct,
};

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

    fn matches_simple_var_type<'x>(
        &self,
        env: &mut Environment<'x>,
        pattern: &VarType,
    ) -> MatchResult {
        let mut results = vec![env.construct_matches_simple_var_type(self.else_value, pattern)];
        for con in &self.conditions {
            results.push(env.construct_matches_simple_var_type(con.value, pattern))
        }
        MatchResult::and(results)
    }

    fn reduce<'x>(&self, env: &mut Environment<'x>, _self_id: ConstructId) -> ConstructId {
        let base = env.reduce(self.base);
        let mut conditions = Vec::new();
        let mut else_value = self.else_value;
        for condition in &self.conditions {
            let pattern = env.reduce(condition.pattern);
            let value = env.reduce(condition.value);
            match env.construct_matches_construct(base, pattern) {
                MatchResult::Match => {
                    else_value = value;
                    // env.substitute(value, &subs);
                    break;
                }
                MatchResult::NoMatch => (),
                MatchResult::Unknown => conditions.push(Condition { pattern, value }),
            }
        }
        else_value = env.reduce(else_value);
        if conditions.len() == 0 {
            else_value
        } else {
            env.push_construct(Box::new(Self {
                base,
                conditions,
                else_value,
            }))
        }
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
