use super::{downcast_construct, substitution::Substitutions, Construct, GenInvResult, ItemId};
use crate::{
    environment::{
        dependencies::DepResult,
        discover_equality::{DeqPriority, DeqResult, DeqSide, Equal},
        invariants::Invariant,
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
    left: ItemId,
    right: ItemId,
    equal: ItemId,
    unequal: ItemId,
}

impl CDecision {
    pub fn new<'x>(left: ItemId, right: ItemId, equal: ItemId, unequal: ItemId) -> Self {
        Self {
            left,
            right,
            equal,
            unequal,
        }
    }

    pub fn left(&self) -> ItemId {
        self.left
    }

    pub fn right(&self) -> ItemId {
        self.right
    }

    pub fn equal(&self) -> ItemId {
        self.equal
    }

    pub fn unequal(&self) -> ItemId {
        self.unequal
    }
}

impl_any_eq_for_construct!(CDecision);

impl Construct for CDecision {
    fn dyn_clone(&self) -> Box<dyn Construct> {
        Box::new(self.clone())
    }

    fn contents<'x>(&self) -> Vec<ItemId> {
        vec![self.left, self.right, self.equal, self.unequal]
    }

    fn generated_invariants<'x>(&self, this: ItemId, env: &mut Environment<'x>) -> GenInvResult {
        let true_invs = env.generated_invariants(self.equal);
        let mut false_invs = env.generated_invariants(self.equal);
        let mut result = Vec::new();
        for true_inv in true_invs {
            for (index, false_inv) in false_invs.clone().into_iter().enumerate() {
                if env.discover_equal(true_inv.statement, false_inv.statement, 4)
                    == Ok(Equal::yes())
                {
                    let mut deps = true_inv.dependencies;
                    deps.insert(this);
                    result.push(Invariant::new(true_inv.statement, deps));
                    false_invs.remove(index);
                    break;
                }
            }
        }
        result
    }

    fn get_dependencies<'x>(&self, env: &mut Environment<'x>) -> DepResult {
        let mut deps = env.get_dependencies(self.left);
        deps.append(env.get_dependencies(self.right));
        deps.append(env.get_dependencies(self.equal));
        deps.append(env.get_dependencies(self.unequal));
        deps
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
pub struct SWithInvariant(pub crate::environment::invariants::Invariant, pub ItemId);

impl Scope for SWithInvariant {
    fn dyn_clone(&self) -> Box<dyn Scope> {
        Box::new(self.clone())
    }

    fn local_lookup_ident<'x>(
        &self,
        _env: &mut Environment<'x>,
        _ident: &str,
    ) -> LookupIdentResult {
        Ok(None)
    }

    fn local_reverse_lookup_ident<'x>(
        &self,
        _env: &mut Environment<'x>,
        _value: ItemId,
    ) -> ReverseLookupIdentResult {
        Ok(None)
    }

    fn local_lookup_invariant<'x>(
        &self,
        env: &mut Environment<'x>,
        invariant: ItemId,
        limit: u32,
    ) -> LookupInvariantResult {
        // No, I don't want
        let _no_subs = NestedSubstitutions::new();
        match env.discover_equal(self.0.statement, invariant, limit)? {
            Equal::Yes(l) if l.len() == 0 => Ok(self.0.clone()),
            Equal::NeedsHigherLimit => Err(LookupInvariantError::MightNotExist),
            Equal::Yes(..) | Equal::No | Equal::Unknown => {
                Err(LookupInvariantError::DefinitelyDoesNotExist)
            }
        }
    }

    fn parent(&self) -> Option<ItemId> {
        Some(self.1)
    }
}
