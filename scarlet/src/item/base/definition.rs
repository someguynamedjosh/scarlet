use std::{any::Any, fmt::Debug};

use super::{
    check::CheckFeature, dependencies::DependenciesFeature, equality::EqualityFeature,
    invariants::InvariantsFeature,
};
use crate::{item::ItemPtr, shared::AnyEq};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ContainmentType {
    /// Use this if computing a term would cause the contained term to be
    /// computed.
    Computational,
    /// Use this when Computational containment is not an accurate description.
    Definitional,
}

pub trait ItemDefinition:
    Any + Debug + AnyEq + CheckFeature + DependenciesFeature + EqualityFeature + InvariantsFeature
{
    fn clone_into_box(&self) -> Box<dyn ItemDefinition>;

    fn contents(&self) -> Vec<(ContainmentType, &ItemPtr)> {
        vec![]
    }
}
