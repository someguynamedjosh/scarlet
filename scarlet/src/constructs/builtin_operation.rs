use super::base::{Construct, ConstructId};
use crate::{environment::Environment, impl_any_eq_for_construct};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BuiltinOperation {
    Sum32U,
    Difference32U,
    Product32U,
    Quotient32U,
    Modulo32U,
    Power32U,

    LessThan32U,
    LessThanOrEqual32U,
    GreaterThan32U,
    GreaterThanOrEqual32U,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CBuiltinOperation {
    pub op: BuiltinOperation,
    pub args: Vec<ConstructId>,
}

impl_any_eq_for_construct!(CBuiltinOperation);

impl Construct for CBuiltinOperation {
    fn dyn_clone(&self) -> Box<dyn Construct> {
        Box::new(self.clone())
    }

    fn reduce<'x>(&self, env: &mut Environment<'x>, _self_id: ConstructId) -> ConstructId {
        let mut args = Vec::new();
        for arg in &self.args {
            args.push(env.reduce(*arg));
        }
        env.push_construct(Box::new(Self { args, ..*self }))
    }
}
