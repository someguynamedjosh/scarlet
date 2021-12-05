use std::convert::TryInto;

use super::{
    as_builtin_value,
    base::{Construct, ConstructId},
    substitution::Substitutions,
    variable::CVariable,
};
use crate::{environment::Environment, impl_any_eq_for_construct, shared::{TripleBool, Id, Pool}};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Unique;
pub type UniquePool = Pool<Unique, 'U'>;
pub type UniqueId = Id<'U'>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CBuiltinValue {
    Unique(UniqueId),
    Bool(bool),
    _32U(u32),
}

macro_rules! impl_conversions {
    ($Variant:ident $ty:ty) => {
        impl TryInto<$ty> for CBuiltinValue {
            type Error = Self;

            fn try_into(self) -> Result<$ty, Self::Error> {
                match self {
                    Self::$Variant(v) => Ok(v),
                    _ => Err(self),
                }
            }
        }

        impl From<$ty> for CBuiltinValue {
            fn from(v: $ty) -> Self {
                Self::$Variant(v)
            }
        }
    };
}

impl_conversions!(Bool bool);
impl_conversions!(_32U u32);

impl_any_eq_for_construct!(CBuiltinValue);

impl Construct for CBuiltinValue {
    fn dyn_clone(&self) -> Box<dyn Construct> {
        Box::new(self.clone())
    }

    fn check<'x>(&self, _env: &mut Environment<'x>) {}

    fn get_dependencies<'x>(&self, _env: &mut Environment<'x>) -> Vec<CVariable> {
        Vec::new()
    }

    fn is_def_equal<'x>(&self, env: &mut Environment<'x>, other: &dyn Construct) -> TripleBool {
        TripleBool::Unknown
    }

    fn substitute<'x>(
        &self,
        env: &mut Environment<'x>,
        _substitutions: &Substitutions,
    ) -> ConstructId {
        env.push_construct(Box::new(self.clone()))
    }
}
