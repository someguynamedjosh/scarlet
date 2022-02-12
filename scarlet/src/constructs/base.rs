use std::{any::Any, collections::HashSet, fmt::Debug};

use super::{
    structt::CPopulatedStruct,
    substitution::{NestedSubstitutions, SubExpr},
    variable::CVariable,
};
use crate::{
    environment::{dependencies::DepResult, CheckResult, Environment, UnresolvedConstructError},
    resolvable::BoxedResolvable,
    scope::Scope,
    shared::{AnyEq, Id, Pool, TripleBool},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Invariant {
    pub statement: ConstructId,
    pub dependencies: HashSet<ConstructId>,
}

impl Invariant {
    pub fn new(statement: ConstructId, dependencies: HashSet<ConstructId>) -> Self {
        Self {
            statement,
            dependencies,
        }
    }
}

#[derive(Debug)]
pub enum ConstructDefinition<'x> {
    Other(ConstructId),
    Resolved(BoxedConstruct),
    Unresolved(BoxedResolvable<'x>),
}

impl<'x> ConstructDefinition<'x> {
    pub fn is_placeholder(&self) -> bool {
        match self {
            Self::Unresolved(resolvable) => resolvable.is_placeholder(),
            _ => false,
        }
    }

    pub fn as_other(&self) -> Option<ConstructId> {
        match self {
            &Self::Other(con) => Some(con),
            _ => None,
        }
    }

    pub fn as_resolved(&self) -> Option<&BoxedConstruct> {
        match self {
            Self::Resolved(con) => Some(con),
            _ => None,
        }
    }
}

impl<'x> From<Box<dyn Construct>> for ConstructDefinition<'x> {
    fn from(input: Box<dyn Construct>) -> Self {
        Self::Resolved(input)
    }
}

impl<'a, 'x> From<&'a ConstructId> for ConstructDefinition<'x> {
    fn from(input: &'a ConstructId) -> Self {
        Self::Other(*input)
    }
}

impl<'x> From<ConstructId> for ConstructDefinition<'x> {
    fn from(input: ConstructId) -> Self {
        Self::Other(input)
    }
}

#[derive(Debug)]
pub struct AnnotatedConstruct<'x> {
    pub definition: ConstructDefinition<'x>,
    pub reduced: ConstructDefinition<'x>,
    pub invariants: Option<Vec<Invariant>>,
    pub scope: Box<dyn Scope>,
}

pub type ConstructPool<'x> = Pool<AnnotatedConstruct<'x>, 'C'>;
pub type ConstructId = Id<'C'>;

pub type GenInvResult = Vec<Invariant>;

pub type BoxedConstruct = Box<dyn Construct>;
pub trait Construct: Any + Debug + AnyEq {
    fn dyn_clone(&self) -> Box<dyn Construct>;

    #[allow(unused_variables)]
    fn check<'x>(
        &self,
        env: &mut Environment<'x>,
        this: ConstructId,
        scope: Box<dyn Scope>,
    ) -> CheckResult {
        Ok(())
    }

    fn get_dependencies<'x>(&self, env: &mut Environment<'x>) -> DepResult;

    #[allow(unused_variables)]
    fn generated_invariants<'x>(
        &self,
        this: ConstructId,
        env: &mut Environment<'x>,
    ) -> GenInvResult {
        vec![]
    }

    #[allow(unused_variables)]
    fn is_def_equal<'x>(
        &self,
        env: &mut Environment<'x>,
        subs: &NestedSubstitutions,
        other: SubExpr,
        recursion_limit: u32,
    ) -> Result<TripleBool, UnresolvedConstructError> {
        Ok(TripleBool::Unknown)
    }

    fn as_def<'x>(&self) -> ConstructDefinition<'x> {
        ConstructDefinition::Resolved(self.dyn_clone())
    }
}

pub fn downcast_construct<T: Construct>(from: &dyn Construct) -> Option<&T> {
    (from as &dyn Any).downcast_ref()
}

pub fn downcast_boxed_construct<T: Construct>(from: Box<dyn Construct>) -> Option<T> {
    (from as Box<dyn Any>).downcast().ok().map(|b| *b)
}

pub fn as_struct(from: &dyn Construct) -> Option<&CPopulatedStruct> {
    downcast_construct(from)
}

pub fn as_variable(from: &dyn Construct) -> Option<&CVariable> {
    downcast_construct(from)
}

#[macro_export]
macro_rules! impl_any_eq_for_construct {
    ($ConstructName:ident) => {
        impl crate::shared::AnyEq for $ConstructName {
            fn eq(&self, other: &dyn crate::shared::AnyEq) -> bool {
                (other as &dyn std::any::Any)
                    .downcast_ref::<Self>()
                    .map(|x| self == x)
                    .unwrap_or(false)
            }
        }
    };
}
