use super::{Construct, ConstructId, GenInvResult, Invariant};
use crate::{
    environment::{
        def_equal::{DefEqualResult, IsDefEqual},
        dependencies::DepResult,
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
                if env.is_def_equal(
                    SubExpr(true_inv.statement, &Default::default()),
                    SubExpr(false_inv.statement, &Default::default()),
                    4,
                ) == Ok(IsDefEqual::Yes)
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

    fn symm_is_def_equal<'x>(
        &self,
        env: &mut Environment<'x>,
        subs: &NestedSubstitutions,
        SubExpr(other, other_subs): SubExpr,
        recursion_limit: u32,
    ) -> DefEqualResult {
        assert_ne!(recursion_limit, 0);
        if let Some(other) = env.get_and_downcast_construct_definition::<Self>(other)? {
            let other = other.clone();
            let parts = vec![
                env.is_def_equal(
                    SubExpr(self.left, subs),
                    SubExpr(other.left, other_subs),
                    recursion_limit - 1,
                )?,
                env.is_def_equal(
                    SubExpr(self.right, subs),
                    SubExpr(other.right, other_subs),
                    recursion_limit - 1,
                )?,
                env.is_def_equal(
                    SubExpr(self.equal, subs),
                    SubExpr(other.equal, other_subs),
                    recursion_limit - 1,
                )?,
                env.is_def_equal(
                    SubExpr(self.unequal, subs),
                    SubExpr(other.unequal, other_subs),
                    recursion_limit - 1,
                )?,
            ];
            Ok(IsDefEqual::and(parts))
        } else {
            Ok(IsDefEqual::NeedsHigherLimit)
        }
    }

    fn asymm_is_def_equal<'x>(
        &self,
        env: &mut Environment<'x>,
        subs: &NestedSubstitutions,
        SubExpr(other, other_subs): SubExpr,
        recursion_limit: u32,
    ) -> DefEqualResult {
        let other = env.get_construct_definition(other)?.dyn_clone();
        Ok(
            match env.is_def_equal(
                SubExpr(self.left, subs),
                SubExpr(self.right, other_subs),
                recursion_limit - 1,
            )? {
                IsDefEqual::Yes => other.symm_is_def_equal(
                    env,
                    other_subs,
                    SubExpr(self.equal, subs),
                    recursion_limit - 1,
                )?,
                IsDefEqual::No => other.symm_is_def_equal(
                    env,
                    other_subs,
                    SubExpr(self.unequal, subs),
                    recursion_limit - 1,
                )?,
                _ => IsDefEqual::Unknowable,
            },
        )
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
        match env.is_def_equal(
            SubExpr(self.0.statement, &no_subs),
            SubExpr(invariant, &no_subs),
            limit,
        )? {
            IsDefEqual::Yes => Ok(self.0.clone()),
            IsDefEqual::NeedsHigherLimit => Err(LookupInvariantError::MightNotExist),
            IsDefEqual::No | IsDefEqual::Unknowable => {
                Err(LookupInvariantError::DefinitelyDoesNotExist)
            }
        }
    }

    fn parent(&self) -> Option<ConstructId> {
        Some(self.1)
    }
}
