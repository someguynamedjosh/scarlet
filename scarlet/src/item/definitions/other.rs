use std::fmt::Debug;

use crate::{
    impl_any_eq_from_regular_eq,
    item::{
        check::CheckFeature,
        dependencies::{Dcc, DepResult, DependenciesFeature, OnlyCalledByDcc},
        equality::{Ecc, EqualResult, EqualityFeature, OnlyCalledByEcc},
        invariants::{Icc, PredicatesFeature, PredicatesResult, OnlyCalledByIcc},
        ContainmentType, ItemDefinition, ItemPtr,
    },
};

#[derive(Clone, PartialEq, Eq)]
pub struct DOther {
    other: ItemPtr,
}

impl Debug for DOther {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(other) ")?;
        self.other.fmt(f)
    }
}

impl DOther {
    pub fn new(other: ItemPtr) -> Self {
        Self { other }
    }

    pub fn other(&self) -> &ItemPtr {
        &self.other
    }
}

impl_any_eq_from_regular_eq!(DOther);

impl ItemDefinition for DOther {
    fn clone_into_box(&self) -> Box<dyn ItemDefinition> {
        Box::new(self.clone())
    }

    fn contents(&self) -> Vec<(ContainmentType, ItemPtr)> {
        vec![(ContainmentType::Computational, self.other.ptr_clone())]
    }
}

impl CheckFeature for DOther {}

impl DependenciesFeature for DOther {
    fn get_dependencies_using_context(
        &self,
        _this: &ItemPtr,
        ctx: &mut Dcc,
        affects_return_value: bool,
        _: OnlyCalledByDcc,
    ) -> DepResult {
        ctx.get_dependencies(&self.other, affects_return_value)
    }
}

impl EqualityFeature for DOther {
    fn get_equality_using_context(&self, ctx: &mut Ecc, _: OnlyCalledByEcc) -> EqualResult {
        let equal = ctx
            .with_primary(self.other.ptr_clone())
            .get_equality_left()?;
        Ok(equal)
    }
}

impl PredicatesFeature for DOther {
    fn get_predicates_using_context(
        &self,
        _this: &ItemPtr,
        ctx: &mut Icc,
        _: OnlyCalledByIcc,
    ) -> PredicatesResult {
        ctx.get_invariants(&self.other)
    }
}
