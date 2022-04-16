mod check;
mod definition;
mod dependencies;
mod invariants;
mod item;
mod util;
mod equality;

use std::{any::Any, cell::RefCell, fmt::Debug, rc::Rc};

pub use self::{check::*, definition::*, dependencies::*, invariants::*, item::*};
use super::{
    resolvable::{BoxedResolvable, DUnresolved, Resolvable},
    structt::CPopulatedStruct,
    substitution::Substitutions,
    variable::{CVariable, VariableId},
};
use crate::{
    environment::{
        discover_equality::{DeqPriority, DeqResult, DeqSide, Equal},
        invariants::{InvariantSet, InvariantSetPtr},
        CheckResult, Environment,
    },
    scope::Scope,
    shared::{AnyEq, Id, Pool},
};

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
