use super::{Construct, ConstructId, GenInvResult};
use crate::{
    environment::{
        def_equal::DefEqualResult,
        dependencies::DepResult,
        sub_expr::{NestedSubstitutions, SubExpr},
        Environment,
    },
    impl_any_eq_for_construct,
    shared::TripleBool,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CIsPopulatedStruct(ConstructId);

impl CIsPopulatedStruct {
    pub fn new<'x>(base: ConstructId) -> Self {
        Self(base)
    }
}

impl_any_eq_for_construct!(CIsPopulatedStruct);

impl Construct for CIsPopulatedStruct {
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
        _env: &mut Environment<'x>,
        _subs: &NestedSubstitutions,
        _other: SubExpr,
        _recursion_limit: u32,
    ) -> DefEqualResult {
        Ok(TripleBool::Unknown)
    }
}
