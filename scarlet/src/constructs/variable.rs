use itertools::Itertools;

use super::{
    base::{Construct, ConstructId},
    downcast_construct,
    substitution::Substitutions,
};
use crate::{
    environment::Environment,
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
    capturing: bool,
    depends_on: Vec<CVariable>,
}

impl CVariable {
    pub fn new<'x>(
        id: VariableId,
        invariants: Vec<ConstructId>,
        capturing: bool,
        depends_on: Vec<CVariable>,
    ) -> Self {
        Self {
            id,
            invariants: invariants.clone(),
            capturing,
            depends_on,
        }
    }

    pub(crate) fn get_id(&self) -> VariableId {
        self.id
    }

    pub(crate) fn get_invariants(&self) -> &[ConstructId] {
        &self.invariants[..]
    }

    pub(crate) fn is_capturing(&self) -> bool {
        self.capturing
    }

    pub fn is_same_variable_as(&self, other: &Self) -> bool {
        self.id == other.id && self.capturing == other.capturing
    }

    pub fn can_be_assigned<'x>(&self, value: ConstructId, env: &mut Environment<'x>) -> bool {
        let mut substitutions = OrderedMap::new();
        substitutions.insert_no_replace(self.clone(), value);
        for inv in &self.invariants {
            let subbed = env.substitute(*inv, &substitutions);
            env.reduce(subbed);
            let subbed = env.resolve(subbed);
            if !env.has_invariant(subbed, value) {
                return false;
            }
        }
        for required_dep in &self.depends_on {
            let mut met = false;
            for value_dep in env.get_dependencies(value) {
                if value_dep.is_same_variable_as(&required_dep) {
                    met = true;
                    break;
                }
            }
            if !met {
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
        this: ConstructId,
        env: &mut Environment<'x>,
    ) -> Vec<ConstructId> {
        self.invariants.clone()
    }

    fn get_dependencies<'x>(&self, env: &mut Environment<'x>) -> Vec<CVariable> {
        vec![self.clone()]
    }

    fn is_def_equal<'x>(&self, env: &mut Environment<'x>, other: &dyn Construct) -> TripleBool {
        if let Some(other) = downcast_construct::<Self>(other) {
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
    ) -> ConstructId {
        for (target, value) in substitutions {
            if target.id == self.id && target.capturing == self.capturing {
                return *value;
            }
        }
        let invariants = self
            .invariants
            .iter()
            .copied()
            .map(|x| env.substitute(x, substitutions))
            .collect_vec();
        Self::new(
            self.id,
            invariants.clone(),
            self.capturing,
            self.depends_on.clone(),
        );
        todo!()
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

    fn local_lookup_invariant<'x>(
        &self,
        env: &mut Environment<'x>,
        invariant: ConstructId,
    ) -> bool {
        false
    }

    fn parent(&self) -> Option<ConstructId> {
        Some(self.0)
    }
}
