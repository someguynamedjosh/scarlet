use super::{
    base::{Construct, ConstructId},
    substitution::{NestedSubstitutions, SubExpr},
    GenInvResult,
};
use crate::{
    environment::{dependencies::DepResult, DefEqualResult, Environment},
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

    fn generated_invariants<'x>(
        &self,
        _this: ConstructId,
        env: &mut Environment<'x>,
    ) -> GenInvResult {
        env.generated_invariants(self.0)
    }

    fn get_dependencies<'x>(&self, env: &mut Environment<'x>) -> DepResult {
        env.get_dependencies(self.0)
    }

    fn is_def_equal<'x>(
        &self,
        env: &mut Environment<'x>,
        subs: &NestedSubstitutions,
        SubExpr(other, other_subs): SubExpr,
        recursion_limit: u32,
    ) -> DefEqualResult {
        let other = env.get_construct_definition(other)?.dyn_clone();
        other.is_def_equal(env, other_subs, SubExpr(self.0, subs), recursion_limit)
    }
}
