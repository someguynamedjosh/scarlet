use itertools::Itertools;
use maplit::hashset;

use super::substitution::Substitutions;
use crate::{
    environment::Environment,
    impl_any_eq_from_regular_eq,
    item::{
        check::CheckFeature,
        dependencies::{Dcc, DepResult, DependenciesFeature, OnlyCalledByDcc},
        equality::{Equal, EqualResult, EqualityFeature},
        invariants::{
            Icc, InvariantSet, InvariantSetPtr, InvariantsFeature, InvariantsResult,
            OnlyCalledByIcc,
        },
        ItemDefinition, ItemPtr,
    },
    scope::{
        LookupIdentResult, LookupInvariantError, LookupInvariantResult, ReverseLookupIdentResult,
        Scope,
    },
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DDecision {
    left: ItemPtr,
    right: ItemPtr,
    equal: ItemPtr,
    unequal: ItemPtr,
}

impl DDecision {
    pub fn new(left: ItemPtr, right: ItemPtr, equal: ItemPtr, unequal: ItemPtr) -> Self {
        Self {
            left,
            right,
            equal,
            unequal,
        }
    }

    pub fn left(&self) -> ItemPtr {
        self.left
    }

    pub fn right(&self) -> ItemPtr {
        self.right
    }

    pub fn equal(&self) -> ItemPtr {
        self.equal
    }

    pub fn unequal(&self) -> ItemPtr {
        self.unequal
    }
}

impl_any_eq_from_regular_eq!(DDecision);

impl ItemDefinition for DDecision {
    fn dyn_clone(&self) -> Box<dyn ItemDefinition> {
        Box::new(self.clone())
    }

    fn contents(&self) -> Vec<ItemPtr> {
        vec![self.left, self.right, self.equal, self.unequal]
    }
}

impl CheckFeature for DDecision {}

impl DependenciesFeature for DDecision {
    fn get_dependencies_using_context(&self, ctx: &mut Dcc, _: OnlyCalledByDcc) -> DepResult {
        let mut deps = ctx.get_dependencies(&self.left);
        deps.append(ctx.get_dependencies(&self.right));
        deps.append(ctx.get_dependencies(&self.equal));
        deps.append(ctx.get_dependencies(&self.unequal));
        deps
    }
}

impl EqualityFeature for DDecision {
    fn get_equality_using_context(
        &self,
        ctx: &mut Environment,
        self_subs: Vec<&Substitutions>,
        other: ItemPtr,
        other_subs: Vec<&Substitutions>,
        limit: u32,
    ) -> EqualResult {
        if let Some(other) = other.downcast() {
            Ok(Equal::and(vec![
                ctx.discover_equal_with_subs(
                    self.left,
                    self_subs.clone(),
                    other.left,
                    other_subs.clone(),
                    limit,
                )?,
                ctx.discover_equal_with_subs(
                    self.right,
                    self_subs.clone(),
                    other.right,
                    other_subs.clone(),
                    limit,
                )?,
                ctx.discover_equal_with_subs(
                    self.equal,
                    self_subs.clone(),
                    other.equal,
                    other_subs.clone(),
                    limit,
                )?,
                ctx.discover_equal_with_subs(
                    self.unequal,
                    self_subs.clone(),
                    other.unequal,
                    other_subs.clone(),
                    limit,
                )?,
            ]))
        } else {
            Ok(Equal::Unknown)
        }
    }
}

impl InvariantsFeature for DDecision {
    fn get_invariants_using_context(
        &self,
        this: &ItemPtr,
        ctx: &mut Icc,
        _: OnlyCalledByIcc,
    ) -> InvariantsResult {
        let true_invs_id = ctx.generated_invariants(self.equal);
        let true_invs = ctx.get_invariant_set(true_invs_id).clone();
        let false_invs_id = ctx.generated_invariants(self.equal);
        let false_invs = ctx.get_invariant_set(false_invs_id).clone();
        let mut result_statements = Vec::new();
        for &true_inv in true_invs.statements() {
            for (index, &false_inv) in false_invs.statements().iter().enumerate() {
                if ctx.discover_equal(true_inv, false_inv, 4) == Ok(Equal::yes()) {
                    result_statements.push(true_inv);
                    break;
                }
            }
        }
        let len = result_statements.len();
        ctx.push_invariant_set(InvariantSet::new_justified_by(
            this,
            result_statements,
            vec![vec![vec![true_invs_id, false_invs_id]]; len],
        ))
    }
}

#[derive(Clone, Debug)]
pub struct SWithInvariant(pub InvariantSetPtr, pub ItemPtr);

impl Scope for SWithInvariant {
    fn dyn_clone(&self) -> Box<dyn Scope> {
        Box::new(self.clone())
    }

    fn local_lookup_ident(&self, _ctx: &mut Environment, _ident: &str) -> LookupIdentResult {
        Ok(None)
    }

    fn local_reverse_lookup_ident(
        &self,
        _env: &mut Environment,
        _value: ItemPtr,
    ) -> ReverseLookupIdentResult {
        Ok(None)
    }

    fn local_get_invariant_sets(&self, env: &mut Environment) -> Vec<InvariantSetPtr> {
        vec![self.0]
    }

    fn parent(&self) -> Option<ItemPtr> {
        Some(self.1)
    }
}
