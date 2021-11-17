use super::base::{Construct, ConstructId};
use crate::{environment::Environment, impl_any_eq_for_construct};

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

    fn reduce<'x>(&self, env: &mut Environment<'x>, _self_id: ConstructId) -> ConstructId {
        let base = env.reduce(self.base);
        let mut conditions = Vec::new();
        for condition in &self.conditions {
            conditions.push(Condition {
                pattern: env.reduce(condition.pattern),
                value: env.reduce(condition.value),
            });
        }
        let else_value = env.reduce(self.else_value);
        env.push_construct(Box::new(Self {
            base,
            conditions,
            else_value,
        }))
    }
}
