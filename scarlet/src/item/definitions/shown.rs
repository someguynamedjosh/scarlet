use crate::{
    environment::Environment,
    impl_any_eq_from_regular_eq,
    item::{
        definitions::{decision::DDecision, substitution::Substitutions},
        dependencies::{Dcc, DepResult, DependenciesFeature, OnlyCalledByDcc},
        equality::{Equal, EqualResult, EqualityFeature},
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
pub struct DShown(ItemPtr);

impl DShown {
    pub fn new(base: ItemPtr) -> Self {
        Self(base)
    }

    pub(crate) fn get_base(&self) -> ItemPtr {
        self.0
    }
}

impl_any_eq_from_regular_eq!(DShown);

impl ItemDefinition for DShown {
    fn dyn_clone(&self) -> Box<dyn ItemDefinition> {
        Box::new(self.clone())
    }

    fn contents(&self) -> Vec<ItemPtr> {
        vec![self.0]
    }
}

impl DependenciesFeature for DShown {
    fn get_dependencies_using_context(&self, ctx: &mut Dcc, _: OnlyCalledByDcc) -> DepResult {
        ctx.get_dependencies(&self.0)
    }
}

impl InvariantsFeature for DShown {
    fn get_invariants_using_context(
        &self,
        this: &ItemPtr,
        ctx: &mut Icc,
        _: OnlyCalledByIcc,
    ) -> InvariantsResult {
        ctx.generated_invariants(self.0)
    }
}
