use crate::{
    impl_any_eq_from_regular_eq,
    item::{
        check::CheckFeature,
        dependencies::{Dcc, DepResult, Dependencies, DependenciesFeature, OnlyCalledByDcc},
        equality::{Ecc, Equal, EqualResult, EqualityFeature, OnlyCalledByEcc, PermissionToRefine},
        invariants::{
            Icc, InvariantSet, InvariantSetPtr, InvariantsFeature, InvariantsResult,
            OnlyCalledByIcc,
        },
        ItemDefinition, ItemPtr,
    },
    shared::{Id, Pool},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DPlaceholder;

impl_any_eq_from_regular_eq!(DPlaceholder);

impl ItemDefinition for DPlaceholder {
    fn clone_into_box(&self) -> Box<dyn ItemDefinition> {
        Box::new(self.clone())
    }
}

impl CheckFeature for DPlaceholder {}

impl DependenciesFeature for DPlaceholder {
    fn get_dependencies_using_context(
        &self,
        this: &ItemPtr,
        ctx: &mut Dcc,
        _: OnlyCalledByDcc,
    ) -> DepResult {
        panic!("Attempted to get dependencies of placeholder.");
    }
}

impl EqualityFeature for DPlaceholder {
    fn get_equality_using_context(
        &self,
        _ctx: &mut Ecc,
        _can_refine: PermissionToRefine,
        _: OnlyCalledByEcc,
    ) -> EqualResult {
        panic!("Attempted to test equality of placeholder.");
    }
}

impl InvariantsFeature for DPlaceholder {
    fn get_invariants_using_context(
        &self,
        _this: &ItemPtr,
        ctx: &mut Icc,
        _: OnlyCalledByIcc,
    ) -> InvariantsResult {
        panic!("Attempted to get invariants of placeholder.");
    }
}
