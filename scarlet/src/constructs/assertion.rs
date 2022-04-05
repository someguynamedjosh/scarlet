use maplit::hashset;

use super::{
    base::Construct, downcast_construct, substitution::Substitutions, GenInvResult, ItemId,
};
use crate::{
    environment::{
        dependencies::DepResult,
        discover_equality::{DeqResult, DeqSide, Equal},
        invariants::Invariant,
        CheckResult, Environment, UnresolvedItemError,
    },
    impl_any_eq_for_construct,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CAssertion {
    statement: ItemId,
}

impl CAssertion {
    pub fn new(statement: ItemId) -> Self {
        Self { statement }
    }
}

impl_any_eq_for_construct!(CAssertion);

impl Construct for CAssertion {
    fn dyn_clone(&self) -> Box<dyn Construct> {
        Box::new(self.clone())
    }

    fn check<'x>(
        &self,
        env: &mut Environment<'x>,
        this: ItemId,
        scope: Box<dyn crate::scope::Scope>,
    ) -> CheckResult {
        env.justify(self.statement, this, 16)
            .map(|_| ())
            .map_err(|_| UnresolvedItemError(this))
    }

    fn generated_invariants<'x>(&self, _this: ItemId, _env: &mut Environment<'x>) -> GenInvResult {
        vec![Invariant::new(self.statement, hashset![])]
    }

    fn get_dependencies<'x>(&self, env: &mut Environment<'x>) -> DepResult {
        env.get_dependencies(self.statement)
    }

    fn discover_equality<'x>(
        &self,
        env: &mut Environment<'x>,
        self_subs: Vec<&Substitutions>,
        other_id: ItemId,
        other: &dyn Construct,
        other_subs: Vec<&Substitutions>,
        limit: u32,
    ) -> DeqResult {
        if let Some(other) = downcast_construct::<Self>(other) {
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
