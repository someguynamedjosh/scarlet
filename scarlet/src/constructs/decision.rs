use super::{downcast_construct, Construct, ConstructId, GenInvResult, Invariant};
use crate::{
    environment::{
        dependencies::DepResult,
        discover_equality::{DeqPriority, DeqResult, DeqSide, Equal},
        sub_expr::{NestedSubstitutions, SubExpr},
        Environment,
    },
    impl_any_eq_for_construct,
    scope::{
        LookupIdentResult, LookupInvariantError, LookupInvariantResult, ReverseLookupIdentResult,
        Scope,
    },
    shared::TripleBool,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CDecision {
    left: ConstructId,
    right: ConstructId,
    equal: ConstructId,
    unequal: ConstructId,
}

impl CDecision {
    pub fn new<'x>(
        left: ConstructId,
        right: ConstructId,
        equal: ConstructId,
        unequal: ConstructId,
    ) -> Self {
        Self {
            left,
            right,
            equal,
            unequal,
        }
    }

    pub fn left(&self) -> ConstructId {
        self.left
    }

    pub fn right(&self) -> ConstructId {
        self.right
    }

    pub fn equal(&self) -> ConstructId {
        self.equal
    }

    pub fn unequal(&self) -> ConstructId {
        self.unequal
    }
}

impl_any_eq_for_construct!(CDecision);

impl Construct for CDecision {
    fn dyn_clone(&self) -> Box<dyn Construct> {
        Box::new(self.clone())
    }

    fn generated_invariants<'x>(
        &self,
        this: ConstructId,
        env: &mut Environment<'x>,
    ) -> GenInvResult {
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

    fn deq_priority<'x>(&self) -> DeqPriority {
        3
    }

    fn discover_equality<'x>(
        &self,
        env: &mut Environment<'x>,
        _other_id: ConstructId,
        other: &dyn Construct,
        limit: u32,
        tiebreaker: DeqSide,
    ) -> DeqResult {
        if let Some(other) = downcast_construct::<Self>(other) {
            Ok(Equal::and(vec![
                env.discover_equal_with_tiebreaker(self.left, other.left, limit, tiebreaker)?,
                env.discover_equal_with_tiebreaker(self.right, other.right, limit, tiebreaker)?,
                env.discover_equal_with_tiebreaker(self.equal, other.equal, limit, tiebreaker)?,
                env.discover_equal_with_tiebreaker(self.unequal, other.unequal, limit, tiebreaker)?,
            ]))
        } else {
            Ok(Equal::Unknown)
        }
    }
}

#[derive(Clone, Debug)]
pub struct SWithInvariant(pub Invariant, pub ConstructId);

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
        _value: ConstructId,
    ) -> ReverseLookupIdentResult {
        Ok(None)
    }

    fn local_lookup_invariant<'x>(
        &self,
        env: &mut Environment<'x>,
        invariant: ConstructId,
        limit: u32,
    ) -> LookupInvariantResult {
        // No, I don't want
        let no_subs = NestedSubstitutions::new();
        match env.discover_equal(self.0.statement, invariant, limit)? {
            Equal::Yes(l, r) => {
                if l.len() == 0 && r.len() == 0 {
                    Ok(self.0.clone())
                } else {
                    Err(LookupInvariantError::DefinitelyDoesNotExist)
                }
            }
            Equal::NeedsHigherLimit => Err(LookupInvariantError::MightNotExist),
            Equal::No | Equal::Unknown => Err(LookupInvariantError::DefinitelyDoesNotExist),
        }
    }

    fn parent(&self) -> Option<ConstructId> {
        Some(self.1)
    }
}
