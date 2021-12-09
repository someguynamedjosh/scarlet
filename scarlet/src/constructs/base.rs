use std::{any::Any, fmt::Debug};

use super::{
    builtin_value::CBuiltinValue, structt::CStruct, substitution::Substitutions,
    variable::CVariable,
};
use crate::{
    environment::Environment,
    shared::{AnyEq, Id, Pool, TripleBool},
    tokens::structure::Token,
};

#[derive(Debug)]
pub enum ConstructDefinition<'x> {
    Placeholder,
    Resolved(BoxedConstruct),
    Unresolved(Token<'x>),
}

impl<'x> ConstructDefinition<'x> {
    pub fn as_resolved(&self) -> Option<&BoxedConstruct> {
        match self {
            Self::Resolved(con) => Some(con),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct AnnotatedConstruct<'x> {
    pub definition: ConstructDefinition<'x>,
    pub parent_scope: Option<ConstructId>,
}

pub type ConstructPool<'x> = Pool<AnnotatedConstruct<'x>, 'C'>;
pub type ConstructId = Id<'C'>;

pub type BoxedConstruct = Box<dyn Construct>;
pub trait Construct: Any + Debug + AnyEq {
    fn dyn_clone(&self) -> Box<dyn Construct>;

    fn check<'x>(&self, env: &mut Environment<'x>);

    fn get_dependencies<'x>(&self, env: &mut Environment<'x>) -> Vec<CVariable>;

    #[allow(unused_variables)]
    fn is_def_equal<'x>(&self, env: &mut Environment<'x>, other: &dyn Construct) -> TripleBool {
        TripleBool::Unknown
    }

    fn substitute<'x>(
        &self,
        env: &mut Environment<'x>,
        substitutions: &Substitutions,
    ) -> ConstructId;
}

pub fn downcast_construct<T: Construct>(from: &dyn Construct) -> Option<&T> {
    (from as &dyn Any).downcast_ref()
}

pub fn as_builtin_value(from: &dyn Construct) -> Option<&CBuiltinValue> {
    downcast_construct(from)
}

pub fn as_struct(from: &dyn Construct) -> Option<&CStruct> {
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
