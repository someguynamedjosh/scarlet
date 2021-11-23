use constructs::{substitution::Substitutions, variable::CVariable};

use super::{
    base::{Construct, ConstructId},
    variable::VarType,
};
use crate::{
    constructs::{self, builtin_value::CBuiltinValue, downcast_construct},
    environment::{matchh::MatchResult, Environment},
    impl_any_eq_for_construct,
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
            if !env
                .construct_matches_simple_var_type(arg, &VarType::_32U)
                .is_guaranteed_match()
            {
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

    fn matches_simple_var_type<'x>(
        &self,
        env: &mut Environment<'x>,
        pattern: &VarType,
    ) -> MatchResult {
        match pattern {
            VarType::Anything => unreachable!(),
            VarType::_32U => match self.op {
                BuiltinOperation::Sum32U
                | BuiltinOperation::Difference32U
                | BuiltinOperation::Product32U
                | BuiltinOperation::Quotient32U
                | BuiltinOperation::Modulo32U
                | BuiltinOperation::Power32U => MatchResult::non_capturing(),
                _ => MatchResult::NoMatch,
            },
            VarType::Bool => match self.op {
                BuiltinOperation::LessThan32U
                | BuiltinOperation::LessThanOrEqual32U
                | BuiltinOperation::GreaterThan32U
                | BuiltinOperation::GreaterThanOrEqual32U => MatchResult::non_capturing(),
                _ => MatchResult::NoMatch,
            },
            VarType::Just(other) => {
                if let Some(pattern_op) = downcast_construct::<Self>(&**env.get_construct(*other)) {
                    let pattern_op = pattern_op.clone();
                    if pattern_op.op == self.op {
                        for (&self_arg, &pattern_arg) in
                            self.args.iter().zip(pattern_op.args.iter())
                        {
                            if !env
                                .construct_matches_construct(self_arg, pattern_arg)
                                .is_guaranteed_match()
                            {
                                return MatchResult::Unknown;
                            }
                        }
                        MatchResult::non_capturing()
                    } else {
                        MatchResult::Unknown
                    }
                } else {
                    MatchResult::Unknown
                }
            }
            VarType::And(_, _) => unreachable!(),
            VarType::Or(_, _) => unreachable!(),
            VarType::Struct { .. } => unreachable!(),
        }
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
