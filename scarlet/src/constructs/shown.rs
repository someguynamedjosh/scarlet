use super::{
    base::{Construct, ConstructId},
    substitution::Substitutions,
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
pub struct CShown(ConstructId);

impl CShown {
    pub fn new<'x>(base: ConstructId) -> Self {
        Self(base)
    }

    pub(crate) fn get_base(&self) -> ConstructId {
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

    fn generated_invariants<'x>(
        &self,
        _this: ConstructId,
        env: &mut Environment<'x>,
    ) -> GenInvResult {
        env.generated_invariants(self.0)
    }

    fn dereference(&self) -> Option<(ConstructId, Option<&Substitutions>)> {
        Some((self.0, None))
    }
}
