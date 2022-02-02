use maplit::hashset;

use super::{
    base::Construct, downcast_construct, substitution::Substitutions, BoxedConstruct,
    ConstructDefinition, ConstructId,
};
use crate::{
    constructs::Invariant,
    environment::{dependencies::Dependencies, Environment},
    impl_any_eq_for_construct,
    scope::Scope,
    shared::{Id, Pool, TripleBool},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CAxiom {
    statement: ConstructId,
}

impl CAxiom {
    fn new(env: &mut Environment, statement: &str) -> Self {
        Self {
            statement: env.get_language_item(statement),
        }
    }

    pub fn from_name(env: &mut Environment, name: &str) -> Self {
        Self::new(env, &format!("{}_statement", name))
    }
}

impl_any_eq_for_construct!(CAxiom);

impl Construct for CAxiom {
    fn dyn_clone(&self) -> Box<dyn Construct> {
        Box::new(self.clone())
    }

    fn generated_invariants<'x>(
        &self,
        this: ConstructId,
        env: &mut Environment<'x>,
    ) -> Vec<Invariant> {
        vec![Invariant::new(self.statement, hashset![])]
    }

    fn get_dependencies<'x>(&self, env: &mut Environment<'x>) -> Dependencies {
        env.get_dependencies(self.statement)
    }

    fn reduce<'x>(&self, env: &mut Environment<'x>) -> ConstructDefinition<'x> {
        env.get_language_item("void").into()
    }

    fn is_def_equal<'x>(&self, env: &mut Environment<'x>, other: &dyn Construct) -> TripleBool {
        if let Some(other) = downcast_construct::<Self>(other) {
            env.is_def_equal(self.statement, other.statement)
        } else {
            TripleBool::Unknown
        }
    }

    fn substitute<'x>(
        &self,
        env: &mut Environment<'x>,
        substitutions: &Substitutions,
    ) -> ConstructDefinition<'x> {
        ConstructDefinition::Resolved(Box::new(Self {
            statement: env.substitute(self.statement, substitutions),
        }))
    }
}
