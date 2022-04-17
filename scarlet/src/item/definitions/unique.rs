use crate::{
    environment::Environment,
    impl_any_eq_from_regular_eq,
    item::{
        definitions::{decision::DDecision, substitution::Substitutions},
        dependencies::{Dcc, DepResult, Dependencies, DependenciesFeature, OnlyCalledByDcc},
        equality::{Equal, EqualResult, EqualityFeature, Ecc},
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
    shared::{Id, Pool},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Unique;
pub type UniquePool = Pool<Unique, 'U'>;
pub type UniqueId = Id<'U'>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DUnique(UniqueId);

impl DUnique {
    pub fn new(id: UniqueId) -> Self {
        Self(id)
    }
}

impl_any_eq_from_regular_eq!(DUnique);

impl ItemDefinition for DUnique {
    fn dyn_clone(&self) -> Box<dyn ItemDefinition> {
        Box::new(self.clone())
    }
}

impl DependenciesFeature for DUnique {}

impl EqualityFeature for DUnique {
    fn get_equality_using_context(&self, ctx: &Ecc) -> EqualResult {
        // Ok(if let Some(other) = downcast_construct::<Self>(other) {
        //     if self.0 == other.0 {
        //         Equal::yes()
        //     } else {
        //         Equal::No
        //     }
        // } else {
        //     Equal::Unknown
        // })
        todo!()
    }
}
