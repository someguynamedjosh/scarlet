use itertools::Itertools;

use super::{
    base::{Construct, ConstructId},
    downcast_construct,
    substitution::{CSubstitution, Substitutions},
};
use crate::{
    environment::Environment,
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

    pub fn is_same_variable_as(&self, env: &mut Environment, other: &Self) -> bool {
        if !(self.id == other.id) {
            return false;
        }
        if self.substitutions.len() != other.substitutions.len() {
            return false;
        }
        for (&left, &right) in self.substitutions.iter().zip(other.substitutions.iter()) {
            if env.is_def_equal(left, right) != TripleBool::True {
                return false;
            }
        }
        true
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
        if deps.len() < self.substitutions.len() {
            return false;
        }
        for (target, &value) in deps.iter().zip(self.substitutions.iter()) {
            if !target.can_be_assigned(value, env) {
                return false;
            }
        }
        true
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

    fn get_dependencies<'x>(&self, env: &mut Environment<'x>) -> Vec<CVariable> {
        let mut deps = Vec::new();
        for &sub in &self.substitutions {
            deps.append(&mut env.get_dependencies(sub));
        }
        deps.push(self.clone());
        deps
    }

    fn is_def_equal<'x>(&self, env: &mut Environment<'x>, other: &dyn Construct) -> TripleBool {
        if let Some(other) = downcast_construct::<Self>(other) {
            if self.is_same_variable_as(env, other) {
                return TripleBool::True;
            }
        }
        TripleBool::Unknown
    }

    fn substitute<'x>(
        &self,
        env: &mut Environment<'x>,
        substitutions: &Substitutions,
        scope: Box<dyn Scope>,
    ) -> ConstructId {
        for (target, value) in substitutions {
            if self.is_same_variable_as(env, target) {
                return *value;
            }
        }
        let invariants = self
            .invariants
            .iter()
            .copied()
            .map(|x| env.substitute(x, substitutions))
            .collect_vec();
        let mut substitutions = self
            .substitutions
            .iter()
            .map(|&sub| env.substitute(sub, substitutions))
            .collect();
        let con = Self::new(self.id, invariants.clone(), substitutions);
        env.push_construct(con, scope)
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
