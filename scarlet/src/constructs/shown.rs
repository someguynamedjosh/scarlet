use super::{
    base::{Construct, ConstructId},
    substitution::Substitutions,
    variable::CVariable,
};
use crate::{environment::Environment, impl_any_eq_for_construct, scope::{SPlain, Scope}};

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

    fn check<'x>(&self, _env: &mut Environment<'x>) {}

    fn generated_invariants<'x>(
        &self,
        this: ConstructId,
        env: &mut Environment<'x>,
    ) -> Vec<ConstructId> {
        env.generated_invariants(self.0)
    }

    fn get_dependencies<'x>(&self, env: &mut Environment<'x>) -> Vec<CVariable> {
        env.get_dependencies(self.0)
    }

    fn substitute<'x>(
        &self,
        env: &mut Environment<'x>,
        substitutions: &Substitutions,
        scope: Box<dyn Scope>,
    ) -> ConstructId {
        let base = env.substitute(self.0, substitutions);
        env.push_construct(Self::new(base), scope)
    }
}
