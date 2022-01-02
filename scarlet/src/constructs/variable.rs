use itertools::Itertools;

use super::{
    base::{Construct, ConstructId},
    downcast_construct,
    substitution::{CSubstitution, Substitutions},
    BoxedConstruct, ConstructDefinition,
};
use crate::{
    environment::{dependencies::Dependencies, Environment},
    impl_any_eq_for_construct,
    scope::{SPlain, SRoot, Scope},
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

    pub fn can_be_assigned<'x>(&self, value: ConstructId, env: &mut Environment<'x>) -> bool {
        let mut substitutions = OrderedMap::new();
        substitutions.insert_no_replace(self.clone(), value);
        for inv in &self.invariants {
            let subbed = env.substitute(*inv, &substitutions);
            env.reduce(subbed);
            if !env.has_invariant(subbed, value) {
                return false;
            }
        }
        let deps = env.get_dependencies(value);
        if deps.num_variables() < self.substitutions.len() {
            return false;
        }
        for (target, &value) in deps.into_variables().zip(self.substitutions.iter()) {
            if !target.can_be_assigned(value, env) {
                return false;
            }
        }
        true
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

    fn check<'x>(&self, _env: &mut Environment<'x>) {}

    fn generated_invariants<'x>(
        &self,
        _this: ConstructId,
        _env: &mut Environment<'x>,
    ) -> Vec<ConstructId> {
        self.invariants.clone()
    }

    fn get_dependencies<'x>(&self, env: &mut Environment<'x>) -> Dependencies {
        let mut deps = Dependencies::new();
        for &sub in &self.substitutions {
            deps.append(env.get_dependencies(sub));
        }
        deps.push_eager(self.clone());
        deps
    }

    fn is_def_equal<'x>(&self, env: &mut Environment<'x>, other: &dyn Construct) -> TripleBool {
        if let Some(other) = downcast_construct::<Self>(other) {
            if self.substitutions.len() != other.substitutions.len() {
                return TripleBool::Unknown;
            }
            for (&left, &right) in self.substitutions.iter().zip(other.substitutions.iter()) {
                if env.is_def_equal(left, right) != TripleBool::True {
                    return TripleBool::Unknown;
                }
            }
            if self.is_same_variable_as(other) {
                return TripleBool::True;
            }
        }
        TripleBool::Unknown
    }

    fn substitute<'x>(
        &self,
        env: &mut Environment<'x>,
        substitutions: &Substitutions,
    ) -> BoxedConstruct {
        for (target, value) in substitutions {
            if self.is_same_variable_as(target) {
                let deps = env.get_dependencies(*value);
                let mut stored_subs = Substitutions::new();
                for (target, &value) in deps.into_variables().zip(self.substitutions.iter()) {
                    stored_subs.insert_no_replace(target.clone(), value);
                }
                let value_def = env.get_construct_definition(*value).dyn_clone();
                return value_def.substitute(env, &stored_subs);
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
        Self::new(self.id, invariants.clone(), substitutions).dyn_clone()
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
    ) -> bool {
        false
    }

    fn parent(&self) -> Option<ConstructId> {
        Some(self.0)
    }
}
