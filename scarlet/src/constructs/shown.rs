use super::{
    base::{Construct, ConstructId},
    downcast_construct,
    substitution::{NestedSubstitutions, SubExpr, Substitutions},
    ConstructDefinition, Invariant,
};
use crate::{
    environment::{dependencies::Dependencies, Environment},
    impl_any_eq_for_construct,
    shared::TripleBool,
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
        subs: &NestedSubstitutions,
        SubExpr(other, other_subs): SubExpr,
    ) -> TripleBool {
        let other = env.get_construct_definition(other).dyn_clone();
        other.is_def_equal(env, other_subs, SubExpr(self.0, subs))
    }

    fn substitute<'x>(
        &self,
        env: &mut Environment<'x>,
        substitutions: &Substitutions,
    ) -> ConstructDefinition<'x> {
        let base = env.substitute(self.0, substitutions);
        base.into()
    }
}
