use std::collections::HashSet;

use crate::item::{
    decision::CDecision, downcast_construct, substitution::Substitutions, ItemDefinition, GenInvResult,
    ItemPtr,
};
use crate::{
    environment::{
        dependencies::DepResult,
        discover_equality::{DeqResult, Equal},
        Environment,
    },
    impl_any_eq_for_construct,
    scope::SPlain,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CIsPopulatedStruct {
    base: ItemPtr,
}

impl CIsPopulatedStruct {
    pub fn new(base: ItemPtr) -> Self {
        Self { base }
    }

    pub fn get_base(&self) -> ItemPtr {
        self.base
    }
}

impl_any_eq_for_construct!(CIsPopulatedStruct);

impl ItemDefinition for CIsPopulatedStruct {
    fn dyn_clone(&self) -> Box<dyn ItemDefinition> {
        Box::new(self.clone())
    }

    fn generated_invariants(&self, this: ItemPtr, env: &mut Environment) -> GenInvResult {
        let invs = env.generated_invariants(self.base);
        let truee = env.get_language_item("true");
        let falsee = env.get_language_item("false");
        let this_is_false = env.push_construct(
            CDecision::new(this, falsee, truee, falsee),
            Box::new(SPlain(this)),
        );
        let is_bool = env.push_construct(
            CDecision::new(this, truee, truee, this_is_false),
            Box::new(SPlain(this)),
        );
        let mut set = env.get_invariant_set(invs).clone();
        set.push(is_bool);
        env.push_invariant_set(set)
    }

    fn get_dependencies(&self, env: &mut Environment) -> DepResult {
        env.get_dependencies(self.base)
    }

    fn discover_equality(
        &self,
        env: &mut Environment,
        self_subs: Vec<&Substitutions>,
        other_id: ItemPtr,
        other: &dyn ItemDefinition,
        other_subs: Vec<&Substitutions>,
        limit: u32,
    ) -> DeqResult {
        if let Some(other) = downcast_construct::<Self>(other) {
            let other = other.clone();
            env.discover_equal_with_subs(self.base, self_subs, other.base, other_subs, limit)
        } else {
            Ok(Equal::Unknown)
        }
    }
}
