pub mod arithmetic;
pub mod index;
pub mod length;

use std::fmt::Debug;

use constructs::{substitution::Substitutions, variable::CVariable};

use super::base::{Construct, ConstructId};
use crate::{
    constructs::{self, builtin_value::CBuiltinValue, downcast_construct},
    environment::Environment,
    impl_any_eq_for_construct,
    shared::{AnyEq, TripleBool},
};

pub trait BuiltinOperation: AnyEq + Debug {
    fn check<'x>(&self, env: &mut Environment<'x>, args: &[ConstructId]);
    fn compute<'x>(&self, env: &mut Environment<'x>, args: &[ConstructId]) -> Option<ConstructId>;
    fn dyn_clone(&self) -> Box<dyn BuiltinOperation>;
}

#[derive(Debug)]
pub struct CBuiltinOperation {
    pub op: Box<dyn BuiltinOperation>,
    pub args: Vec<ConstructId>,
}

impl Clone for CBuiltinOperation {
    fn clone(&self) -> Self {
        Self {
            op: self.op.dyn_clone(),
            args: self.args.clone(),
        }
    }
}

impl PartialEq for CBuiltinOperation {
    fn eq(&self, other: &Self) -> bool {
        for (larg, rarg) in self.args.iter().zip(other.args.iter()) {
            if larg != rarg {
                return false;
            }
        }
        self.op.eq(&*other.op)
    }
}

impl_any_eq_for_construct!(CBuiltinOperation);

impl Construct for CBuiltinOperation {
    fn dyn_clone(&self) -> Box<dyn Construct> {
        Box::new(self.clone())
    }

    fn check<'x>(&self, env: &mut Environment<'x>) {
        self.op.check(env, &self.args[..])
    }

    fn get_dependencies<'x>(&self, env: &mut Environment<'x>) -> Vec<CVariable> {
        let mut deps = Vec::new();
        for arg in &self.args {
            deps.append(&mut env.get_dependencies(*arg));
        }
        deps
    }

    fn is_def_equal<'x>(&self, env: &mut Environment<'x>, other: &dyn Construct) -> TripleBool {
        TripleBool::Unknown
    }

    fn reduce<'x>(&self, env: &mut Environment<'x>, _self_id: ConstructId) -> ConstructId {
        let mut args = Vec::new();
        for arg in &self.args {
            args.push(env.reduce(*arg));
        }
        if let Some(result) = self.op.compute(env, &args[..]) {
            result
        } else {
            env.push_construct(Box::new(Self {
                op: self.op.dyn_clone(),
                args,
            }))
        }
    }

    fn substitute<'x>(
        &self,
        env: &mut Environment<'x>,
        substitutions: &Substitutions,
    ) -> ConstructId {
        let mut args = Vec::new();
        for arg in &self.args {
            args.push(env.substitute(*arg, substitutions));
        }
        let def = Self {
            op: self.op.dyn_clone(),
            args,
        };
        env.push_construct(Box::new(def))
    }
}
