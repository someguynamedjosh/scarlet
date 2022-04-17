use crate::{
    environment::Environment,
    impl_any_eq_from_regular_eq,
    item::{
        check::CheckFeature,
        definitions::{decision::DDecision, substitution::Substitutions},
        dependencies::{Dcc, DepResult, DependenciesFeature, OnlyCalledByDcc},
        equality::{Ecc, Equal, EqualResult, EqualityFeature, OnlyCalledByEcc, PermissionToRefine},
        invariants::{
            Icc, InvariantSet, InvariantSetPtr, InvariantsFeature, InvariantsResult,
            OnlyCalledByIcc,
        },
        ItemDefinition, ItemPtr,
    },
    scope::{
        LookupIdentResult, LookupInvariantError, LookupInvariantResult, ReverseLookupIdentResult,
        SPlain, Scope,
    },
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DRecursion(ItemPtr);

impl DRecursion {
    pub fn new(base: ItemPtr) -> Self {
        Self(base)
    }

    pub(crate) fn get_base(&self) -> ItemPtr {
        self.0
    }
}

impl_any_eq_from_regular_eq!(DRecursion);

impl ItemDefinition for DRecursion {
    fn clone_into_box(&self) -> Box<dyn ItemDefinition> {
        Box::new(self.clone())
    }
}

impl CheckFeature for DRecursion {}

impl DependenciesFeature for DRecursion {
    fn get_dependencies_using_context(&self, ctx: &mut Dcc, _: OnlyCalledByDcc) -> DepResult {
        ctx.get_dependencies(&self.0)
    }
}

impl EqualityFeature for DRecursion {
    fn get_equality_using_context(
        &self,
        ctx: &Ecc,
        can_refine: PermissionToRefine,
        _: OnlyCalledByEcc,
    ) -> EqualResult {
        unreachable!();
    }
}

impl InvariantsFeature for DRecursion {
    fn get_invariants_using_context(
        &self,
        this: &ItemPtr,
        ctx: &mut Icc,
        _: OnlyCalledByIcc,
    ) -> InvariantsResult {
        self.0.get_invariants()
    }
}
