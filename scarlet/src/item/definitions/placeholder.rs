use crate::{
    impl_any_eq_from_regular_eq,
    item::{
        check::CheckFeature,
        dependencies::{Dcc, DepResult, DependenciesFeature, OnlyCalledByDcc},
        equality::{Ecc, EqualResult, EqualityFeature, OnlyCalledByEcc},
        invariants::{Icc, InvariantsFeature, InvariantsResult, OnlyCalledByIcc},
        ItemDefinition, ItemPtr,
    },
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DPlaceholder {
    pub name: String,
}

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
        _this: &ItemPtr,
        _ctx: &mut Dcc,
        _affects_return_value: bool,
        _: OnlyCalledByDcc,
    ) -> DepResult {
        panic!(
            "Attempted to get dependencies of placeholder '{}'.",
            self.name
        );
    }
}

impl EqualityFeature for DPlaceholder {
    fn get_equality_using_context(&self, _ctx: &mut Ecc, _: OnlyCalledByEcc) -> EqualResult {
        panic!("Attempted to test equality of placeholder '{}'.", self.name);
    }
}

impl InvariantsFeature for DPlaceholder {
    fn get_invariants_using_context(
        &self,
        _this: &ItemPtr,
        _ctx: &mut Icc,
        _: OnlyCalledByIcc,
    ) -> InvariantsResult {
        panic!(
            "Attempted to get invariants of placeholder '{}'.",
            self.name
        );
    }
}
