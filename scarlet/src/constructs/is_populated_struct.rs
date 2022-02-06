use super::{
    downcast_construct, structt::CPopulatedStruct, substitution::{Substitutions, NestedSubstitutions, SubExpr}, unique::CUnique,
    Construct, ConstructDefinition, ConstructId, Invariant,
};
use crate::{
    environment::{dependencies::Dependencies, Environment},
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
    ) -> Vec<Invariant> {
        env.generated_invariants(self.0)
    }

    fn get_dependencies<'x>(&self, env: &mut Environment<'x>) -> Dependencies {
        env.get_dependencies(self.0)
    }

    fn is_def_equal<'x>(
        &self,
        _env: &mut Environment<'x>,
        subs: &NestedSubstitutions,
        other: SubExpr,
        recursion_limit: u32,
    ) -> TripleBool {
        TripleBool::Unknown
    }
}
