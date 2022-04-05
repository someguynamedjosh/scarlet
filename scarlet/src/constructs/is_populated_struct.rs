use std::collections::HashSet;

use super::{decision::CDecision, Construct, GenInvResult, ItemId};
use crate::{
    environment::{dependencies::DepResult, invariants::Invariant, Environment},
    impl_any_eq_for_construct,
    scope::SPlain,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CIsPopulatedStruct {
    base: ItemId,
}

impl CIsPopulatedStruct {
    pub fn new<'x>(base: ItemId) -> Self {
        Self { base }
    }

    pub fn get_base(&self) -> ItemId {
        self.base
    }
}

impl_any_eq_for_construct!(CIsPopulatedStruct);

impl Construct for CIsPopulatedStruct {
    fn dyn_clone(&self) -> Box<dyn Construct> {
        Box::new(self.clone())
    }

    fn generated_invariants<'x>(&self, this: ItemId, env: &mut Environment<'x>) -> GenInvResult {
        let mut invs = env.generated_invariants(self.base);
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
        invs.push(Invariant {
            statement: is_bool,
            dependencies: HashSet::new(),
        });
        invs
    }

    fn get_dependencies<'x>(&self, env: &mut Environment<'x>) -> DepResult {
        env.get_dependencies(self.base)
    }
}
