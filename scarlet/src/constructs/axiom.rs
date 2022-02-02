use maplit::hashset;

use super::{
    base::Construct,
    downcast_construct,
    substitution::{NestedSubstitutions, SubExpr, Substitutions},
    BoxedConstruct, ConstructDefinition, ConstructId,
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
        _this: ConstructId,
        _env: &mut Environment<'x>,
    ) -> Vec<Invariant> {
        vec![Invariant::new(self.statement, hashset![])]
    }

    fn get_dependencies<'x>(&self, env: &mut Environment<'x>) -> Dependencies {
        env.get_dependencies(self.statement)
    }

    fn is_def_equal<'x>(
        &self,
        env: &mut Environment<'x>,
        subs: &NestedSubstitutions,
        other: SubExpr,
    ) -> TripleBool {
        let other_subs = other.1;
        if let Some(other) = env.get_and_downcast_construct_definition::<Self>(other.0) {
            let other = other.clone();
            env.is_def_equal(
                SubExpr(self.statement, subs),
                SubExpr(other.statement, other_subs),
            )
        } else {
            TripleBool::Unknown
        }
    }
}
