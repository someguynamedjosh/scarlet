use super::{
    base::{Construct, ItemId},
    substitution::Substitutions,
    variable::VariableId,
    GenInvResult,
};
use crate::{
    environment::{
        dependencies::DepResult,
        discover_equality::{DeqResult, DeqSide},
        Environment,
    },
    impl_any_eq_for_construct,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CShown(ItemId);

impl CShown {
    pub fn new<'x>(base: ItemId) -> Self {
        Self(base)
    }

    pub(crate) fn get_base(&self) -> ItemId {
        self.0
    }
}

impl_any_eq_for_construct!(CShown);

impl Construct for CShown {
    fn dyn_clone(&self) -> Box<dyn Construct> {
        Box::new(self.clone())
    }

    fn get_dependencies<'x>(&self, env: &mut Environment<'x>) -> DepResult {
        env.get_dependencies(self.0)
    }

    fn generated_invariants<'x>(&self, _this: ItemId, env: &mut Environment<'x>) -> GenInvResult {
        env.generated_invariants(self.0)
    }

    fn dereference(
        &self,
        env: &mut Environment,
    ) -> Option<(ItemId, Option<&Substitutions>, Option<Vec<VariableId>>)> {
        Some((self.0, None, None))
    }
}
