use maplit::hashset;

use super::{
    base::{Construct, ConstructId},
    substitution::{NestedSubstitutions, SubExpr, Substitutions},
    GenInvResult, Invariant,
};
use crate::{
    environment::{
        dependencies::{DepResult, Dependencies},
        DefEqualResult, Environment, UnresolvedConstructError,
    },
    impl_any_eq_for_construct,
    scope::{LookupIdentResult, LookupInvariantResult, ReverseLookupIdentResult, Scope},
    shared::{Id, Pool, TripleBool},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variable {
    pub id: Option<VariableId>,
    pub invariants: Vec<ConstructId>,
    pub dependencies: Vec<ConstructId>,
}
pub type VariablePool = Pool<Variable, 'V'>;
pub type VariableId = Id<'V'>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Dependency {
    pub id: VariableId,
    pub swallow: Vec<VariableId>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CVariable(pub VariableId);

impl CVariable {
    pub fn new<'x>(id: VariableId) -> Self {
        Self(id)
    }

    pub(crate) fn get_id(&self) -> VariableId {
        self.0
    }

    pub fn is_same_variable_as(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Variable {
    pub(crate) fn get_invariants(&self) -> &[ConstructId] {
        &self.invariants[..]
    }

    pub(crate) fn get_dependencies(&self) -> &[ConstructId] {
        &self.dependencies
    }

    pub fn can_be_assigned<'x>(
        &self,
        value: ConstructId,
        env: &mut Environment<'x>,
        other_subs: &Substitutions,
        limit: u32,
    ) -> Result<Result<Vec<Invariant>, String>, UnresolvedConstructError> {
        let mut substitutions = other_subs.clone();
        let mut invariants = Vec::new();
        substitutions.insert_no_replace(self.id.unwrap(), value);
        for &inv in &self.invariants {
            let subbed = env.substitute(inv, &substitutions);
            if let Some(inv) = env.get_produced_invariant(subbed, value, limit) {
                invariants.push(inv);
            } else {
                return Ok(Err(format!(
                    "Failed to find invariant: {}",
                    env.show(subbed, value)?
                )));
            }
        }
        Ok(Ok(invariants))
    }

    pub fn as_dependency(&self, env: &mut Environment) -> Dependency {
        let mut deps = Dependencies::new();
        for &dep in &self.dependencies {
            deps.append(env.get_dependencies(dep));
        }
        Dependency {
            id: self.id.unwrap(),
            swallow: deps.as_variables().map(|x| x.id).collect(),
        }
    }
}

impl_any_eq_for_construct!(CVariable);

impl Construct for CVariable {
    fn dyn_clone(&self) -> Box<dyn Construct> {
        Box::new(self.clone())
    }

    fn generated_invariants<'x>(
        &self,
        this: ConstructId,
        env: &mut Environment<'x>,
    ) -> GenInvResult {
        env.get_variable(self.0)
            .invariants
            .iter()
            .map(|&i| Invariant::new(i, hashset![this]))
            .collect()
    }

    fn get_dependencies<'x>(&self, env: &mut Environment<'x>) -> DepResult {
        let mut deps = Dependencies::new();
        for dep in env.get_variable(self.0).dependencies.clone() {
            deps.append(env.get_dependencies(dep));
        }
        deps.push_eager(env.get_variable(self.0).clone().as_dependency(env));
        for inv in env.get_variable(self.0).invariants.clone() {
            deps.append(env.get_dependencies(inv));
        }
        deps
    }

    fn is_def_equal<'x>(
        &self,
        env: &mut Environment<'x>,
        subs: &NestedSubstitutions,
        SubExpr(other, other_subs): SubExpr,
        recursion_limit: u32,
    ) -> DefEqualResult {
        assert_ne!(recursion_limit, 0);
        for (target, value) in subs {
            if *target == self.0 {
                let mut new_subs = value.1.clone();
                for (target, value) in subs {
                    if new_subs.contains_key(target) {
                        if new_subs.get(target).unwrap() != value {
                            eprintln!("{:#?}", env);
                            todo!("{:#?}, {:?} -> {:?}", new_subs, target, value);
                        }
                    } else {
                        new_subs.insert_no_replace(target.clone(), *value);
                    }
                }
                return env.is_def_equal(
                    SubExpr(value.0, &new_subs),
                    SubExpr(other, other_subs),
                    recursion_limit,
                );
            }
        }
        if let Some(other) = env.get_and_downcast_construct_definition::<Self>(other)? {
            let other = other.clone();
            if other_subs.iter().any(|(key, _)| *key == other.get_id()) {
                return Ok(TripleBool::Unknown);
            }
            if self.is_same_variable_as(&other) {
                return Ok(TripleBool::True);
            }
        }
        Ok(TripleBool::Unknown)
    }
}

#[derive(Debug, Clone)]
pub struct SVariableInvariants(pub ConstructId);

impl Scope for SVariableInvariants {
    fn dyn_clone(&self) -> Box<dyn Scope> {
        Box::new(self.clone())
    }

    fn local_lookup_ident<'x>(&self, _env: &mut Environment<'x>, ident: &str) -> LookupIdentResult {
        Ok(if ident == "SELF" { Some(self.0) } else { None })
    }

    fn local_reverse_lookup_ident<'a, 'x>(
        &self,
        _env: &'a mut Environment<'x>,
        value: ConstructId,
    ) -> ReverseLookupIdentResult {
        Ok(if value == self.0 {
            Some("SELF".to_owned())
        } else {
            None
        })
    }

    fn local_lookup_invariant<'x>(
        &self,
        _env: &mut Environment<'x>,
        _invariant: ConstructId,
        _limit: u32,
    ) -> LookupInvariantResult {
        Ok(None)
    }

    fn parent(&self) -> Option<ConstructId> {
        Some(self.0)
    }
}
