use std::{any::Any, fmt::Debug};

use crate::{
    shared::{AnyEq, Id, Pool},
    tokens::structure::Token,
};

#[derive(Debug)]
pub enum ConstructDefinition<'x> {
    Placeholder,
    Resolved(BoxedConstruct),
    Unresolved(Token<'x>),
}

#[derive(Debug)]
pub struct AnnotatedConstruct<'x> {
    pub definition: ConstructDefinition<'x>,
    pub parent_scope: Option<ConstructId>,
}

pub type ConstructPool<'x> = Pool<AnnotatedConstruct<'x>, 'C'>;
pub type ConstructId = Id<'C'>;

pub type BoxedConstruct = Box<dyn Construct>;
pub trait Construct: Any + Debug + AnyEq {}

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
