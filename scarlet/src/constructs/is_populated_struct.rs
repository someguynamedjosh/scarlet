use super::{
    downcast_construct, structt::CPopulatedStruct, substitution::Substitutions, unique::CUnique,
    Construct, ConstructDefinition, ConstructId,
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

    fn check<'x>(&self, _env: &mut Environment<'x>) {}

    fn generated_invariants<'x>(
        &self,
        _this: ConstructId,
        env: &mut Environment<'x>,
    ) -> Vec<ConstructId> {
        env.generated_invariants(self.0)
    }

    fn get_dependencies<'x>(&self, env: &mut Environment<'x>) -> Dependencies {
        env.get_dependencies(self.0)
    }

    fn is_def_equal<'x>(&self, _env: &mut Environment<'x>, _other: &dyn Construct) -> TripleBool {
        TripleBool::Unknown
    }

    fn reduce<'x>(&self, env: &mut Environment<'x>) -> ConstructDefinition<'x> {
        let base = env.get_reduced_construct_definition(self.0);
        if downcast_construct::<CPopulatedStruct>(&**base).is_some() {
            env.get_language_item("true").into()
        } else if downcast_construct::<CUnique>(&**base).is_some() {
            env.get_language_item("false").into()
        } else {
            self.dyn_clone().into()
        }
    }

    fn substitute<'x>(
        &self,
        env: &mut Environment<'x>,
        substitutions: &Substitutions,
    ) -> Box<dyn Construct> {
        let base = env.substitute(self.0, substitutions);
        Self::new(base).dyn_clone()
    }
}
