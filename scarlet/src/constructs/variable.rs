use maplit::hashset;

use super::{
    base::{Construct, ItemId},
    downcast_construct,
    substitution::Substitutions,
    GenInvResult,
};
use crate::{
    environment::{
        dependencies::{DepResult, Dependencies},
        discover_equality::{DeqPriority, DeqResult, Equal},
        invariants::InvariantSet,
        Environment,
    },
    impl_any_eq_for_construct,
    scope::{
        LookupIdentResult, LookupInvariantError, LookupInvariantResult, ReverseLookupIdentResult,
        Scope,
    },
    shared::{Id, Pool},
};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct VariableOrder {
    /// Explicitly defined order, 0-255.
    pub major_order: u8,
    /// Implicit order by which file it's in.
    file_order: u32,
    /// Implicit order by position in file.
    minor_order: u32,
}

impl VariableOrder {
    pub fn new(major_order: u8, file_order: u32, minor_order: u32) -> Self {
        Self {
            major_order,
            file_order,
            minor_order,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variable {
    pub id: Option<VariableId>,
    pub item: Option<ItemId>,
    pub invariants: Vec<ItemId>,
    pub dependencies: Vec<ItemId>,
    pub order: VariableOrder,
}
pub type VariablePool = Pool<Variable, 'V'>;
pub type VariableId = Id<'V'>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Dependency {
    pub id: VariableId,
    pub swallow: Vec<VariableId>,
    pub order: VariableOrder,
}

impl PartialOrd for Dependency {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.order.partial_cmp(&other.order)
    }
}

impl Ord for Dependency {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.order.cmp(&other.order)
    }
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
    pub(crate) fn get_invariants(&self) -> &[ItemId] {
        &self.invariants[..]
    }

    pub(crate) fn get_dependencies(&self) -> &[ItemId] {
        &self.dependencies
    }

    pub(crate) fn get_var_dependencies(&self, env: &mut Environment) -> Dependencies {
        let mut result = Dependencies::new();
        for &dep in &self.dependencies {
            result.append(env.get_dependencies(dep));
        }
        result
    }

    pub fn assignment_justifications<'x>(
        &self,
        value: ItemId,
        env: &mut Environment<'x>,
        other_subs: &Substitutions,
        limit: u32,
    ) -> Vec<ItemId> {
        let mut substitutions = other_subs.clone();
        let mut justifications = Vec::new();
        substitutions.insert_no_replace(self.id.unwrap(), value);
        for &inv in &self.invariants {
            let subbed = env.substitute_unchecked(inv, &substitutions);
            justifications.push(subbed);
        }
        justifications
    }

    pub fn as_dependency(&self, env: &mut Environment) -> Dependency {
        let mut deps = Dependencies::new();
        for &dep in &self.dependencies {
            deps.append(env.get_dependencies(dep));
        }
        Dependency {
            id: self.id.unwrap(),
            swallow: deps.as_variables().map(|x| x.id).collect(),
            order: self.order.clone(),
        }
    }
}

impl_any_eq_for_construct!(CVariable);

impl Construct for CVariable {
    fn dyn_clone(&self) -> Box<dyn Construct> {
        Box::new(self.clone())
    }

    fn generated_invariants<'x>(&self, this: ItemId, env: &mut Environment<'x>) -> GenInvResult {
        let statements = env.get_variable(self.0).invariants.clone();
        let justification = vec![this];
        env.push_invariant_set(InvariantSet::new(statements, justification))
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

    fn discover_equality<'x>(
        &self,
        env: &mut Environment<'x>,
        self_subs: Vec<&Substitutions>,
        other_id: ItemId,
        other: &dyn Construct,
        other_subs: Vec<&Substitutions>,
        limit: u32,
    ) -> DeqResult {
        unreachable!()
    }
}

#[derive(Debug, Clone)]
pub struct SVariableInvariants(pub ItemId);

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
        value: ItemId,
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
        _invariant: ItemId,
        _limit: u32,
    ) -> LookupInvariantResult {
        Err(LookupInvariantError::DefinitelyDoesNotExist)
    }

    fn parent(&self) -> Option<ItemId> {
        Some(self.0)
    }
}
