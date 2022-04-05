use super::{Construct, GenInvResult, ItemId};
use crate::{
    environment::{dependencies::DepResult, Environment},
    impl_any_eq_for_construct,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CIsPopulatedStruct(ItemId);

impl CIsPopulatedStruct {
    pub fn new<'x>(base: ItemId) -> Self {
        Self(base)
    }

    pub fn get_base(&self) -> ItemId {
        self.0
    }
}

impl_any_eq_for_construct!(CIsPopulatedStruct);

impl Construct for CIsPopulatedStruct {
    fn dyn_clone(&self) -> Box<dyn Construct> {
        Box::new(self.clone())
    }

    fn generated_invariants<'x>(&self, _this: ItemId, env: &mut Environment<'x>) -> GenInvResult {
        env.generated_invariants(self.0)
    }

    fn get_dependencies<'x>(&self, env: &mut Environment<'x>) -> DepResult {
        env.get_dependencies(self.0)
    }
}
