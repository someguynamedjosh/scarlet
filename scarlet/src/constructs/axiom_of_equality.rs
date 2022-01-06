use super::{
    base::Construct, downcast_construct, substitution::Substitutions, BoxedConstruct, ConstructId,
};
use crate::{
    environment::{dependencies::Dependencies, Environment},
    impl_any_eq_for_construct,
    shared::{Id, Pool, TripleBool},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CAxiom(ConstructId);

impl CAxiom {
    pub fn axiom_of_equality(env: &mut Environment) -> Self {
        Self(env.get_language_item("ae_statement"))
    }
}

impl_any_eq_for_construct!(CAxiom);

impl Construct for CAxiom {
    fn dyn_clone(&self) -> Box<dyn Construct> {
        Box::new(self.clone())
    }

    fn check<'x>(&self, _env: &mut Environment<'x>) {}

    fn generated_invariants<'x>(
        &self,
        this: ConstructId,
        env: &mut Environment<'x>,
    ) -> Vec<ConstructId> {
        vec![self.0]
    }

    fn get_dependencies<'x>(&self, env: &mut Environment<'x>) -> Dependencies {
        env.get_dependencies(self.0)
    }

    fn is_def_equal<'x>(&self, env: &mut Environment<'x>, other: &dyn Construct) -> TripleBool {
        if let Some(other) = downcast_construct::<Self>(other) {
            env.is_def_equal(self.0, other.0)
        } else {
            TripleBool::Unknown
        }
    }

    fn substitute<'x>(
        &self,
        env: &mut Environment<'x>,
        substitutions: &Substitutions,
    ) -> BoxedConstruct {
        Box::new(Self(env.substitute(self.0, substitutions)))
    }
}
