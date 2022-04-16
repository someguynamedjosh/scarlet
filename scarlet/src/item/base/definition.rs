use std::{any::Any, cell::RefCell, fmt::Debug, rc::Rc};

use super::{
    check::CheckFeature, DepResult, DependenciesFeature, DependencyCalculationContext,
    InvariantsFeature, ItemPtr,
};
use crate::{
    environment::{
        discover_equality::{DeqPriority, DeqResult, DeqSide, Equal},
        invariants::{InvariantSet, InvariantSetPtr},
        CheckResult, Environment,
    },
    item::{
        resolvable::{BoxedResolvable, DUnresolved, Resolvable},
        structt::CPopulatedStruct,
        substitution::Substitutions,
        variable::{CVariable, VariableId},
    },
    scope::Scope,
    shared::{AnyEq, Id, Pool},
};

pub trait ItemDefinition:
    Any + Debug + AnyEq + CheckFeature + DependenciesFeature + InvariantsFeature
{
    fn dyn_clone(&self) -> Box<dyn ItemDefinition>;

    fn contents(&self) -> Vec<&ItemPtr> {
        vec![]
    }

    #[allow(unused_variables)]
    fn discover_equality(
        &self,
        env: &mut Environment,
        self_subs: Vec<&Substitutions>,
        other_id: ItemPtr,
        other: &dyn ItemDefinition,
        other_subs: Vec<&Substitutions>,
        limit: u32,
    ) -> DeqResult {
        Ok(Equal::Unknown)
    }
}
