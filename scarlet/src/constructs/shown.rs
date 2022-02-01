use super::{
    base::{Construct, ConstructId},
    substitution::Substitutions,
    ConstructDefinition, Invariant, downcast_construct,
};
use crate::{
    environment::{dependencies::Dependencies, Environment},
    impl_any_eq_for_construct, shared::TripleBool,
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

    fn generated_invariants<'x>(
        &self,
        _this: ConstructId,
        env: &mut Environment<'x>,
    ) -> Vec<Invariant> {
        env.generated_invariants(self.0)
    }

    fn get_dependencies<'x>(&self, env: &mut Environment<'x>) -> Dependencies {
        env.get_dependencies(self.0)
    }

    fn is_def_equal<'x>(
        &self,
        env: &mut Environment<'x>,
        other: &dyn Construct,
    ) -> TripleBool {
        if let Some(other) = downcast_construct::<Self>(other) {
            env.is_def_equal(self.0, other.0)
        } else {
            TripleBool::Unknown
        }
    }

    fn reduce<'x>(&self, _env: &mut Environment<'x>) -> ConstructDefinition<'x> {
        self.0.into()
    }

    fn substitute<'x>(
        &self,
        env: &mut Environment<'x>,
        substitutions: &Substitutions,
    ) -> ConstructDefinition<'x> {
        let base = env.substitute(self.0, substitutions);
        ConstructDefinition::Resolved(Self::new(base).dyn_clone())
    }
}
