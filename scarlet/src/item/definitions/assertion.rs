use maplit::hashset;

use crate::{
    environment::{
        dependencies::DepResult,
        discover_equality::{DeqResult, DeqSide, Equal},
        CheckResult, Environment, UnresolvedItemError,
    },
    impl_any_eq_for_construct,
    item::{base::ItemDefinition, substitution::Substitutions, GenInvResult, ItemPtr},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CAssertion {
    statement: ItemPtr,
}

impl CAssertion {
    pub fn new(statement: ItemPtr) -> Self {
        Self { statement }
    }
}

impl_any_eq_for_construct!(CAssertion);

impl ItemDefinition for CAssertion {
    fn dyn_clone(&self) -> Box<dyn ItemDefinition> {
        Box::new(self.clone())
    }

    fn check(
        &self,
        env: &mut Environment,
        this: ItemPtr,
        scope: Box<dyn crate::scope::Scope>,
    ) -> CheckResult {
        todo!()
    }

    fn generated_invariants(&self, _this: ItemPtr, _env: &mut Environment) -> GenInvResult {
        todo!()
    }

    fn get_dependencies(&self, env: &mut Environment) -> DepResult {
        env.get_dependencies(self.statement)
    }

    fn discover_equality(
        &self,
        env: &mut Environment,
        self_subs: Vec<&Substitutions>,
        other_id: ItemPtr,
        other: &dyn ItemDefinition,
        other_subs: Vec<&Substitutions>,
        limit: u32,
    ) -> DeqResult {
        if let Some(other) = other.downcast::<Self>() {
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
