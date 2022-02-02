use itertools::Itertools;
use maplit::hashset;

use super::{
    base::{Construct, ConstructId},
    downcast_construct,
    substitution::{NestedSubstitutions, SubExpr, Substitutions},
    BoxedConstruct, ConstructDefinition, Invariant,
};
use crate::{
    environment::{dependencies::Dependencies, Environment},
    impl_any_eq_for_construct,
    parser::ParseContext,
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
    substitutions: Vec<ConstructId>,
}

impl CVariable {
    pub fn new<'x>(
        id: VariableId,
        invariants: Vec<ConstructId>,
        substitutions: Vec<ConstructId>,
    ) -> Self {
        Self {
            id,
            invariants: invariants.clone(),
            substitutions,
        }
    }

    pub(crate) fn get_id(&self) -> VariableId {
        self.id
    }

    pub(crate) fn get_invariants(&self) -> &[ConstructId] {
        &self.invariants[..]
    }

    pub(crate) fn get_substitutions(&self) -> &[ConstructId] {
        &self.substitutions[..]
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
        for inv in &self.invariants {
            let subbed = env.substitute(*inv, &substitutions);
            if let Some(inv) = env.get_produced_invariant(subbed, value) {
                invariants.push(inv);
            } else {
                return Err(format!(
                    "Failed to find invariant: {}",
                    env.show(subbed, value)
                ));
            }
        }
        let deps = env.get_dependencies(value);
        if deps.num_variables() < self.substitutions.len() {
            return Err(format!(
                "Expected at least {} dependencies, got {} instead.",
                self.substitutions.len(),
                deps.num_variables(),
            ));
        }
        for (target, &value) in deps.into_variables().zip(self.substitutions.iter()) {
            let value_vom = "todo";
            target
                .can_be_assigned(value, env, &Substitutions::new())
                .map_err(|err| format!("while substituting {}:\n{}", value_vom, err))?;
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
        let substitutions = self
            .substitutions
            .iter()
            .map(|&sub| env.substitute(sub, substitutions))
            .collect();
        Some(Self::new(self.id, invariants.clone(), substitutions))
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
        for &sub in &self.substitutions {
            deps.append(env.get_dependencies(sub));
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
                if self.substitutions.len() != 0 {
                    todo!()
                }
                return env.is_def_equal(*value, SubExpr(other, other_subs));
            }
        }
        if let Some(other) = env.get_and_downcast_construct_definition::<Self>(other) {
            let other = other.clone();
            if self.substitutions.len() != other.substitutions.len() {
                return TripleBool::Unknown;
            }
            for (&left, &right) in self.substitutions.iter().zip(other.substitutions.iter()) {
                if env.is_def_equal(SubExpr(left, subs), SubExpr(right, other_subs))
                    != TripleBool::True
                {
                    return TripleBool::Unknown;
                }
            }
            if self.is_same_variable_as(&other) {
                return TripleBool::True;
            }
        }
        TripleBool::Unknown
    }

    fn substitute<'x>(
        &self,
        env: &mut Environment<'x>,
        substitutions: &Substitutions,
    ) -> ConstructDefinition<'x> {
        for (target, value) in substitutions {
            let value = *value;
            if self.is_same_variable_as(target) {
                let deps = env.get_dependencies(value);
                let mut stored_subs = Substitutions::new();
                for (target, &value) in deps.into_variables().zip(self.substitutions.iter()) {
                    let value = env.substitute(value, substitutions);
                    stored_subs.insert_no_replace(target.clone(), value);
                }
                return env.substitute(value, &stored_subs).into();
            }
        }
        let invariants = self
            .invariants
            .iter()
            .copied()
            .map(|x| env.substitute(x, substitutions))
            .collect_vec();
        let substitutions = self
            .substitutions
            .iter()
            .map(|&sub| env.substitute(sub, substitutions))
            .collect();
        ConstructDefinition::Resolved(Self::new(self.id, invariants, substitutions).dyn_clone())
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
