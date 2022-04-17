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
pub struct DIsPopulatedStruct {
    base: ItemPtr,
}

impl DIsPopulatedStruct {
    pub fn new(base: ItemPtr) -> Self {
        Self { base }
    }

    pub fn get_base(&self) -> ItemPtr {
        self.base
    }
}

impl_any_eq_from_regular_eq!(DIsPopulatedStruct);

impl ItemDefinition for DIsPopulatedStruct {
    fn dyn_clone(&self) -> Box<dyn ItemDefinition> {
        Box::new(self.clone())
    }
}

impl InvariantsFeature for DIsPopulatedStruct {
    fn get_invariants_using_context(
        &self,
        this: &ItemPtr,
        ctx: &mut Icc,
        _: OnlyCalledByIcc,
    ) -> InvariantsResult {
        let invs = ctx.generated_invariants(self.base);
        let truee = ctx.get_language_item("true");
        let falsee = ctx.get_language_item("false");
        let this_is_false = ctx.push_construct(
            DDecision::new(this, falsee, truee, falsee),
            Box::new(SPlain(this)),
        );
        let is_bool = ctx.push_construct(
            DDecision::new(this, truee, truee, this_is_false),
            Box::new(SPlain(this)),
        );
        let mut set = ctx.get_invariant_set(invs).clone();
        set.push(is_bool);
        ctx.push_invariant_set(set)
    }
}

impl DependenciesFeature for DIsPopulatedStruct {
    fn get_dependencies_using_context(&self, ctx: &mut Dcc, _: OnlyCalledByDcc) -> DepResult {
        ctx.get_dependencies(&self.base)
    }
}

impl EqualityFeature for DIsPopulatedStruct {
    fn get_equality_using_context(
        &self,
        env: &mut Environment,
        self_subs: Vec<&Substitutions>,
        other: ItemPtr,
        other_subs: Vec<&Substitutions>,
        limit: u32,
    ) -> EqualResult {
        if let Some(other) = other.downcast() {
            let other = other.clone();
            env.discover_equal_with_subs(self.base, self_subs, other.base, other_subs, limit)
        } else {
            Ok(Equal::Unknown)
        }
    }
}
