use maplit::hashset;

use super::substitution::Substitutions;
use crate::{
    environment::Environment,
    impl_any_eq_from_regular_eq,
    item::{
        check::CheckFeature,
        dependencies::{Dcc, DepResult, DependenciesFeature, OnlyCalledByDcc},
        equality::{Equal, EqualResult, EqualityFeature},
        invariants::{Icc, InvariantSet, InvariantsFeature, InvariantsResult, OnlyCalledByIcc},
        ItemDefinition, ItemPtr,
    },
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DAxiom {
    statement: ItemPtr,
}

impl DAxiom {
    fn new(env: &mut Environment, statement: &str) -> Self {
        Self {
            statement: env.get_language_item(statement),
        }
    }

    pub fn from_name(env: &mut Environment, name: &str) -> Self {
        Self::new(env, &format!("{}_statement", name))
    }

    pub fn get_statement(&self, env: &mut Environment) -> &'static str {
        for limit in 0..32 {
            for lang_item_name in env.language_item_names() {
                let lang_item = env.get_language_item(lang_item_name);
                if env.discover_equal(self.statement, lang_item, limit) == Ok(Equal::yes()) {
                    return lang_item_name;
                }
            }
        }
        panic!("{:?} is not an axiom statement", self.statement)
    }
}

impl_any_eq_from_regular_eq!(DAxiom);

impl ItemDefinition for DAxiom {
    fn dyn_clone(&self) -> Box<dyn ItemDefinition> {
        Box::new(self.clone())
    }
}

impl CheckFeature for DAxiom {}

impl DependenciesFeature for DAxiom {
    fn get_dependencies_using_context(&self, ctx: &mut Dcc, _: OnlyCalledByDcc) -> DepResult {
        ctx.get_dependencies(&self.statement)
    }
}

impl EqualityFeature for DAxiom {
    fn get_equality_using_context(
        &self,
        env: &mut Environment,
        self_subs: Vec<&Substitutions>,
        other: ItemPtr,
        other_subs: Vec<&Substitutions>,
        limit: u32,
    ) -> EqualResult {
        if let Some(other) = other.downcast() {
            let other = other.clone();
            env.discover_equal_with_subs(
                self.statement,
                self_subs,
                other.statement,
                other_subs,
                limit,
            )
        } else {
            Ok(Equal::Unknown)
        }
    }
}

impl InvariantsFeature for DAxiom {
    fn get_invariants_using_context(
        &self,
        this: &ItemPtr,
        ctx: &mut Icc,
        _: OnlyCalledByIcc,
    ) -> InvariantsResult {
        Ok(InvariantSet::new(
            this,
            vec![self.statement],
            vec![],
            hashset![],
        ))
    }
}
