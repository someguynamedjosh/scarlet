use std::{any::Any, cell::RefCell, fmt::Debug, rc::Rc};

use super::{
    check::CheckFeature, equality::EqualityFeature, DepResult, DependenciesFeature,
    DependencyCalculationContext, InvariantsFeature, ItemPtr,
};
use crate::{environment::Environment, item::substitution::Substitutions, shared::AnyEq};

pub trait ItemDefinition:
    Any + Debug + AnyEq + CheckFeature + DependenciesFeature + EqualityFeature + InvariantsFeature
{
    fn dyn_clone(&self) -> Box<dyn ItemDefinition>;

    fn contents(&self) -> Vec<&ItemPtr> {
        vec![]
    }
}
