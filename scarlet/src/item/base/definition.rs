use std::{any::Any, cell::RefCell, fmt::Debug, rc::Rc};

use super::{
    check::CheckFeature, dependencies::DependenciesFeature, equality::EqualityFeature,
    invariants::InvariantsFeature,
};
use crate::{
    environment::Environment,
    item::{definitions::substitution::Substitutions, ItemPtr},
    shared::AnyEq,
};

pub trait ItemDefinition:
    Any + Debug + AnyEq + CheckFeature + DependenciesFeature + EqualityFeature + InvariantsFeature
{
    fn clone_into_box(&self) -> Box<dyn ItemDefinition>;

    fn contents(&self) -> Vec<&ItemPtr> {
        vec![]
    }
}
