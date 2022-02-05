use itertools::Itertools;
use maplit::hashset;

use super::{
    base::{Construct, ConstructId},
    substitution::{NestedSubstitutions, SubExpr, Substitutions},
    Invariant,
};
use crate::{
    environment::{dependencies::Dependencies, Environment},
    impl_any_eq_for_construct,
    scope::Scope,
    shared::{Id, OrderedMap, Pool, TripleBool},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variable;
pub type VariablePool = Pool<Variable, 'V'>;
pub type VariableId = Id<'V'>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CVariable {
    id: VariableId,
    invariants: Vec<ConstructId>,
    dependencies: Vec<ConstructId>,
}

impl CVariable {
    pub fn new<'x>(
        id: VariableId,
        invariants: Vec<ConstructId>,
        dependencies: Vec<ConstructId>,
    ) -> Self {
        Self {
            id,
            invariants: invariants.clone(),
            dependencies,
        }
    }

    pub(crate) fn get_id(&self) -> VariableId {
        self.id
    }

    pub(crate) fn get_invariants(&self) -> &[ConstructId] {
        &self.invariants[..]
    }

    pub(crate) fn get_dependencies(&self) -> &[ConstructId] {
        &self.dependencies
    }

    pub fn is_same_variable_as(&self, other: &Self) -> bool {
        self.id == other.id
    }

    pub fn can_be_assigned<'x>(
        &self,
        value: ConstructId,
        env: &mut Environment<'x>,
        other_subs: &Substitutions,
    ) -> Result<Vec<Invariant>, String> {
        let mut substitutions = other_subs.clone();
        let mut invariants = Vec::new();
        substitutions.insert_no_replace(self.clone(), value);
        for &inv in &self.invariants {
            let subbed = env.substitute(inv, &substitutions);
            if let Some(inv) = env.get_produced_invariant(subbed, value) {
                invariants.push(inv);
            } else {
                return Err(format!(
                    "Failed to find invariant: {}",
                    env.show(subbed, value)
                ));
            }
        }
        Ok(invariants)
    }

    pub fn inline_substitute(
        &self,
        env: &mut Environment,
        substitutions: &Substitutions,
    ) -> Option<Self> {
        for (target, _) in substitutions {
            if self.is_same_variable_as(target) {
                return None;
            }
        }
        let invariants = self
            .invariants
            .iter()
            .copied()
            .map(|x| env.substitute(x, substitutions))
            .collect_vec();
        let dependencies = self
            .dependencies
            .iter()
            .map(|value| env.substitute(*value, substitutions))
            .collect();
        Some(Self::new(self.id, invariants.clone(), dependencies))
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
        _env: &mut Environment<'x>,
    ) -> Vec<Invariant> {
        self.invariants
            .iter()
            .map(|&i| Invariant::new(i, hashset![this]))
            .collect()
    }

    fn get_dependencies<'x>(&self, env: &mut Environment<'x>) -> Dependencies {
        let mut deps = Dependencies::new();
        for &dep in &self.dependencies {
            deps.append(env.get_dependencies(dep));
        }
        deps.push_eager(self.clone());
        for &inv in &self.invariants {
            deps.append(env.get_dependencies(inv));
        }
        deps
    }

    fn is_def_equal<'x>(
        &self,
        env: &mut Environment<'x>,
        subs: &NestedSubstitutions,
        SubExpr(other, other_subs): SubExpr,
    ) -> TripleBool {
        for (target, value) in subs {
            if target.is_same_variable_as(self) {
                let mut new_subs = value.1.clone();
                for (target, value) in subs {
                    if new_subs.contains_key(target) {
                        if new_subs.get(target).unwrap() != value {
                            println!("{:#?}", env);
                            todo!("{:#?}, {:?} -> {:?}", new_subs, target, value);
                        }
                    } else {
                        new_subs.insert_no_replace(target.clone(), *value);
                    }
                }
                return env.is_def_equal(SubExpr(value.0, &new_subs), SubExpr(other, other_subs));
            }
        }
        if let Some(other) = env.get_and_downcast_construct_definition::<Self>(other) {
            let other = other.clone();
            if other_subs
                .iter()
                .any(|(key, _)| key.is_same_variable_as(&other))
            {
                return TripleBool::Unknown;
            }
            if self.is_same_variable_as(&other) {
                return TripleBool::True;
            }
        }
        TripleBool::Unknown
    }
}

#[derive(Debug, Clone)]
pub struct SVariableInvariants(pub ConstructId);

impl Scope for SVariableInvariants {
    fn dyn_clone(&self) -> Box<dyn Scope> {
        Box::new(self.clone())
    }

    fn local_lookup_ident<'x>(
        &self,
        _env: &mut Environment<'x>,
        ident: &str,
    ) -> Option<ConstructId> {
        if ident == "SELF" {
            Some(self.0)
        } else {
            None
        }
    }

    fn local_reverse_lookup_ident<'a, 'x>(
        &self,
        _env: &'a mut Environment<'x>,
        value: ConstructId,
    ) -> Option<String> {
        if value == self.0 {
            Some("SELF".to_owned())
        } else {
            None
        }
    }

    fn local_lookup_invariant<'x>(
        &self,
        _env: &mut Environment<'x>,
        _invariant: ConstructId,
    ) -> Option<Invariant> {
        None
    }

    fn parent(&self) -> Option<ConstructId> {
        Some(self.0)
    }
}
