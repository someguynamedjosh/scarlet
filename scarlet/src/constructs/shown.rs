use super::{
    base::{Construct, ConstructId},
    substitution::Substitutions,
    variable::CVariable,
};
use crate::{environment::Environment, impl_any_eq_for_construct};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CShown(pub ConstructId);

impl_any_eq_for_construct!(CShown);

impl Construct for CShown {
    fn dyn_clone(&self) -> Box<dyn Construct> {
        Box::new(self.clone())
    }

    fn check<'x>(&self, _env: &mut Environment<'x>) {}

    fn get_dependencies<'x>(&self, env: &mut Environment<'x>) -> Vec<CVariable> {
        env.get_dependencies(self.0)
    }

    fn substitute<'x>(
        &self,
        env: &mut Environment<'x>,
        substitutions: &Substitutions,
    ) -> ConstructId {
        let base = env.substitute(self.0, substitutions);
        env.push_construct(Box::new(Self(base)))
    }
}
