use itertools::Itertools;

use super::{
    downcast_construct, structt::CPopulatedStruct, substitution::Substitutions, unique::CUnique,
    variable::CVariable, Construct, ConstructDefinition, ConstructId,
};
use crate::{environment::Environment, impl_any_eq_for_construct, shared::TripleBool};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CIsPopulatedStruct(pub ConstructId);

impl_any_eq_for_construct!(CIsPopulatedStruct);

impl Construct for CIsPopulatedStruct {
    fn dyn_clone(&self) -> Box<dyn Construct> {
        Box::new(self.clone())
    }

    fn check<'x>(&self, _env: &mut Environment<'x>) {}

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
    ) -> ConstructId {
        let base = env.substitute(self.0, substitutions);
        env.push_construct(Box::new(Self(base)), vec![base])
    }
}
