use itertools::Itertools;
use maplit::hashset;

use crate::item::{downcast_construct, substitution::Substitutions, ItemDefinition, GenInvResult, ItemPtr};
use crate::{
    environment::{
        dependencies::DepResult,
        discover_equality::{DeqPriority, DeqResult, DeqSide, Equal},
        invariants::{InvariantSet, InvariantSetPtr},
        sub_expr::NestedSubstitutions,
        Environment,
    },
    impl_any_eq_for_construct,
    scope::{
        LookupIdentResult, LookupInvariantError, LookupInvariantResult, ReverseLookupIdentResult,
        Scope,
    },
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CDecision {
    left: ItemPtr,
    right: ItemPtr,
    equal: ItemPtr,
    unequal: ItemPtr,
}

impl CDecision {
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

impl_any_eq_for_construct!(CDecision);

impl ItemDefinition for CDecision {
    fn dyn_clone(&self) -> Box<dyn ItemDefinition> {
        Box::new(self.clone())
    }

    fn contents(&self) -> Vec<ItemPtr> {
        vec![self.left, self.right, self.equal, self.unequal]
    }

    fn generated_invariants(&self, this: ItemPtr, env: &mut Environment) -> GenInvResult {
        let true_invs_id = env.generated_invariants(self.equal);
        let true_invs = env.get_invariant_set(true_invs_id).clone();
        let false_invs_id = env.generated_invariants(self.equal);
        let false_invs = env.get_invariant_set(false_invs_id).clone();
        let mut result_statements = Vec::new();
        for &true_inv in true_invs.statements() {
            for (index, &false_inv) in false_invs.statements().iter().enumerate() {
                if env.discover_equal(true_inv, false_inv, 4) == Ok(Equal::yes()) {
                    result_statements.push(true_inv);
                    break;
                }
            }
        }
        let len = result_statements.len();
        env.push_invariant_set(InvariantSet::new_justified_by(
            this,
            result_statements,
            vec![vec![vec![true_invs_id, false_invs_id]]; len],
        ))
    }

    fn get_dependencies(&self, env: &mut Environment) -> DepResult {
        let mut deps = env.get_dependencies(self.left);
        deps.append(env.get_dependencies(self.right));
        deps.append(env.get_dependencies(self.equal));
        deps.append(env.get_dependencies(self.unequal));
        deps
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
        // println!("{:?} = {:?}", self, other);
        if let Some(other) = downcast_construct::<Self>(other) {
            Ok(Equal::and(vec![
                env.discover_equal_with_subs(
                    self.left,
                    self_subs.clone(),
                    other.left,
                    other_subs.clone(),
                    limit,
                )?,
                env.discover_equal_with_subs(
                    self.right,
                    self_subs.clone(),
                    other.right,
                    other_subs.clone(),
                    limit,
                )?,
                env.discover_equal_with_subs(
                    self.equal,
                    self_subs.clone(),
                    other.equal,
                    other_subs.clone(),
                    limit,
                )?,
                env.discover_equal_with_subs(
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

#[derive(Clone, Debug)]
pub struct SWithInvariant(pub InvariantSetPtr, pub ItemPtr);

impl Scope for SWithInvariant {
    fn dyn_clone(&self) -> Box<dyn Scope> {
        Box::new(self.clone())
    }

    fn local_lookup_ident(
        &self,
        _env: &mut Environment,
        _ident: &str,
    ) -> LookupIdentResult {
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
