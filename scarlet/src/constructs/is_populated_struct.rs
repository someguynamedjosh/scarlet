use itertools::Itertools;

use super::{
    downcast_construct, structt::CPopulatedStruct, substitution::Substitutions, unique::CUnique,
    variable::CVariable, Construct, ConstructDefinition, ConstructId,
};
use crate::{
    environment::Environment,
    impl_any_eq_for_construct,
    scope::{SPlain, Scope},
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
        this: ConstructId,
        env: &mut Environment<'x>,
    ) -> Vec<ConstructId> {
        env.generated_invariants(self.0)
    }

    fn get_dependencies<'x>(&self, env: &mut Environment<'x>) -> Vec<CVariable> {
        env.get_dependencies(self.0)
    }

    fn is_def_equal<'x>(&self, _env: &mut Environment<'x>, _other: &dyn Construct) -> TripleBool {
        TripleBool::Unknown
    }

    fn reduce<'x>(&self, env: &mut Environment<'x>) -> ConstructDefinition<'x> {
        let con = env.get_construct_definition(self.0);
        if downcast_construct::<CPopulatedStruct>(&**con).is_some() {
            env.get_builtin_item("true").into()
        } else if downcast_construct::<CUnique>(&**con).is_some() {
            env.get_builtin_item("false").into()
        } else {
            self.dyn_clone().into()
        }
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
