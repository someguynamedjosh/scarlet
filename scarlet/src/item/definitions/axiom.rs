use maplit::hashset;

use super::substitution::Substitutions;
use crate::{
    environment::Environment,
    impl_any_eq_from_regular_eq,
    item::{
        check::CheckFeature,
        dependencies::{Dcc, DepResult, DependenciesFeature, OnlyCalledByDcc},
        equality::{Ecc, Equal, EqualResult, EqualityFeature, OnlyCalledByEcc, PermissionToRefine},
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
            statement: env.get_language_item(statement).ptr_clone(),
        }
    }

    pub fn from_name(env: &mut Environment, name: &str) -> Self {
        Self::new(env, &format!("{}_statement", name))
    }

    pub fn get_statement(&self, env: &mut Environment) -> &'static str {
        for limit in 0..32 {
            for lang_item_name in env.language_item_names() {
                let lang_item = env.get_language_item(lang_item_name);
                if self.statement.get_equality(&lang_item, limit) == Ok(Equal::yes()) {
                    return lang_item_name;
                }
            }
        }
        panic!("{:?} is not an axiom statement", self.statement)
    }
}

impl_any_eq_from_regular_eq!(DAxiom);

impl ItemDefinition for DAxiom {
    fn clone_into_box(&self) -> Box<dyn ItemDefinition> {
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
        ctx: &mut Ecc,
        can_refine: PermissionToRefine,
        _: OnlyCalledByEcc,
    ) -> EqualResult {
        let statements = if let Some(other) = ctx.rhs().downcast_definition::<Self>() {
            let self_statement = self.statement.ptr_clone();
            let other_statement = other.statement.ptr_clone();
            Some((self_statement, other_statement))
        } else {
            None
        };
        if let Some((self_statement, other_statement)) = statements {
            ctx.refine_and_get_equality(self_statement, other_statement, can_refine)
        } else {
            Ok(Equal::Unknown)
        }
    }
}

impl InvariantsFeature for DAxiom {
    fn get_invariants_using_context(
        &self,
        this: &ItemPtr,
        _ctx: &mut Icc,
        _: OnlyCalledByIcc,
    ) -> InvariantsResult {
        Ok(InvariantSet::new(
            this.ptr_clone(),
            vec![self.statement.ptr_clone()],
            vec![],
            hashset![],
        ))
    }
}
