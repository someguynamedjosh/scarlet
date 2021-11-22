use itertools::Itertools;

use super::{
    base::{Construct, ConstructId},
    downcast_construct,
    substitution::Substitutions,
};
use crate::{
    environment::{matchh::MatchResult, Environment},
    impl_any_eq_for_construct,
    shared::{Id, Pool},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VarType {
    Anything,
    _32U,
    Bool,
    Just(ConstructId),
    And(ConstructId, ConstructId),
    Or(ConstructId, ConstructId),
    Array {
        length: ConstructId,
        eltype: ConstructId,
    },
}

impl VarType {
    pub fn get_dependencies(&self, env: &mut Environment) -> Vec<CVariable> {
        match self {
            VarType::Anything | VarType::_32U | VarType::Bool => vec![],
            VarType::Just(base) => env.get_dependencies(*base),
            VarType::And(l, r)
            | VarType::Or(l, r)
            | VarType::Array {
                length: l,
                eltype: r,
            } => [env.get_dependencies(*l), env.get_dependencies(*r)].concat(),
        }
        .into_iter()
        .filter(|x| !x.capturing)
        .collect_vec()
    }

    pub fn reduce<'x>(self, env: &mut Environment<'x>) -> Self {
        match self {
            Self::And(l, r) => Self::And(env.reduce(l), env.reduce(r)),
            Self::Or(l, r) => Self::Or(env.reduce(l), env.reduce(r)),
            Self::Array { length, eltype } => Self::Array {
                length: env.reduce(length),
                eltype: env.reduce(eltype),
            },
            Self::Just(just) => Self::Just(env.reduce(just)),
            Self::Anything => Self::Anything,
            Self::Bool => Self::Bool,
            Self::_32U => Self::_32U,
        }
    }

    pub fn substitute<'x>(self, env: &mut Environment<'x>, substitutions: &Substitutions) -> Self {
        match self {
            Self::And(l, r) => Self::And(
                env.substitute(l, substitutions),
                env.substitute(r, substitutions),
            ),
            Self::Or(l, r) => Self::Or(
                env.substitute(l, substitutions),
                env.substitute(r, substitutions),
            ),
            Self::Array { length, eltype } => Self::Array {
                length: env.substitute(length, substitutions),
                eltype: env.substitute(eltype, substitutions),
            },
            Self::Just(just) => Self::Just(env.substitute(just, substitutions)),
            Self::Anything => Self::Anything,
            Self::Bool => Self::Bool,
            Self::_32U => Self::_32U,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variable;
pub type VariablePool = Pool<Variable, 'V'>;
pub type VariableId = Id<'V'>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CVariable {
    pub id: VariableId,
    pub typee: VarType,
    pub capturing: bool,
}

impl CVariable {
    pub fn is_same_variable_as(&self, other: &Self) -> bool {
        self.id == other.id && self.capturing == other.capturing
    }
}

impl_any_eq_for_construct!(CVariable);

impl Construct for CVariable {
    fn dyn_clone(&self) -> Box<dyn Construct> {
        Box::new(self.clone())
    }

    fn check<'x>(&self, _env: &mut Environment<'x>) {}

    fn get_dependencies<'x>(&self, env: &mut Environment<'x>) -> Vec<CVariable> {
        let mut base = self.typee.get_dependencies(env);
        base.push(self.clone());
        base
    }

    fn matches_simple_var_type<'x>(
        &self,
        env: &mut Environment<'x>,
        pattern: &VarType,
    ) -> MatchResult {
        if let &VarType::Just(other) = pattern {
            if let Some(pattern_var) = downcast_construct::<Self>(&**env.get_construct(other)) {
                if self.is_same_variable_as(pattern_var) {
                    return MatchResult::non_capturing();
                }
            }
        }
        env.var_type_matches_var_type(&self.typee, pattern)
    }

    fn reduce<'x>(&self, env: &mut Environment<'x>, _self_id: ConstructId) -> ConstructId {
        let def = Self {
            typee: self.typee.clone().reduce(env),
            ..self.clone()
        };
        env.push_construct(Box::new(def))
    }

    fn substitute<'x>(
        &self,
        env: &mut Environment<'x>,
        substitutions: &Substitutions,
    ) -> ConstructId {
        for (target, value) in substitutions {
            if target.id == self.id && target.capturing == self.capturing {
                return *value;
            }
        }
        let typee = self.typee.clone().substitute(env, substitutions);
        let new = Self {
            capturing: self.capturing,
            id: self.id,
            typee,
        };
        env.push_construct(Box::new(new))
    }
}
