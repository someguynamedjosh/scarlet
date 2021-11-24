use constructs::{substitution::Substitutions, variable::CVariable};

use super::base::{Construct, ConstructId};
use crate::{
    constructs::{self, builtin_value::CBuiltinValue, downcast_construct},
    environment::{ Environment},
    impl_any_eq_for_construct,
    shared::TripleBool,
};

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

    fn check<'x>(&self, env: &mut Environment<'x>) {
        for &arg in &self.args {
            if !(todo!("arg always matches 32U") as bool) {
                todo!("Nice error, args must match 32U");
            }
        }
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
        let args_as_values: Option<Vec<_>> = args
            .iter()
            .map(|arg| constructs::as_builtin_value(&**env.get_construct(*arg)).map(|x| *x))
            .collect();
        if let Some(aav) = args_as_values {
            let as_32us: Option<Vec<_>> = aav.iter().map(|x| x.as_32u()).collect();
            let val = match self.op {
                BuiltinOperation::Sum32U => {
                    let as_32us = as_32us.unwrap();
                    CBuiltinValue::_32U(as_32us[0] + as_32us[1])
                }
                BuiltinOperation::Difference32U => {
                    let as_32us = as_32us.unwrap();
                    CBuiltinValue::_32U(as_32us[0] + as_32us[1])
                }
                BuiltinOperation::Product32U => {
                    let as_32us = as_32us.unwrap();
                    CBuiltinValue::_32U(as_32us[0] + as_32us[1])
                }
                BuiltinOperation::Quotient32U => {
                    let as_32us = as_32us.unwrap();
                    CBuiltinValue::_32U(as_32us[0] + as_32us[1])
                }
                BuiltinOperation::Modulo32U => {
                    let as_32us = as_32us.unwrap();
                    CBuiltinValue::_32U(as_32us[0] + as_32us[1])
                }
                BuiltinOperation::Power32U => {
                    let as_32us = as_32us.unwrap();
                    CBuiltinValue::_32U(as_32us[0] + as_32us[1])
                }

                BuiltinOperation::LessThan32U => {
                    let as_32us = as_32us.unwrap();
                    CBuiltinValue::Bool(as_32us[0] < as_32us[1])
                }
                BuiltinOperation::LessThanOrEqual32U => {
                    let as_32us = as_32us.unwrap();
                    CBuiltinValue::Bool(as_32us[0] <= as_32us[1])
                }
                BuiltinOperation::GreaterThan32U => {
                    let as_32us = as_32us.unwrap();
                    CBuiltinValue::Bool(as_32us[0] > as_32us[1])
                }
                BuiltinOperation::GreaterThanOrEqual32U => {
                    let as_32us = as_32us.unwrap();
                    CBuiltinValue::Bool(as_32us[0] >= as_32us[1])
                }
            };
            env.push_construct(Box::new(val))
        } else {
            env.push_construct(Box::new(Self { args, ..*self }))
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
        let def = Self { op: self.op, args };
        env.push_construct(Box::new(def))
    }
}
