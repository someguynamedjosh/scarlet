use maplit::hashset;

use crate::{
    environment::Environment,
    impl_any_eq_from_regular_eq,
    item::{
        definitions::{decision::DDecision, substitution::Substitutions},
        dependencies::{
            Dcc, DepResult, Dependencies, DependenciesFeature, Dependency, OnlyCalledByDcc,
        },
        equality::{Ecc, Equal, EqualResult, EqualityFeature},
        invariants::{
            Icc, InvariantSet, InvariantSetPtr, InvariantsFeature, InvariantsResult,
            OnlyCalledByIcc,
        },
        ItemDefinition, ItemPtr,
    },
    scope::{
        LookupIdentResult, LookupInvariantError, LookupInvariantResult, ReverseLookupIdentResult,
        SPlain, Scope,
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
    pub item: Option<ItemPtr>,
    pub invariants: Vec<ItemPtr>,
    pub dependencies: Vec<ItemPtr>,
    pub order: VariableOrder,
}
pub type VariablePool = Pool<Variable, 'V'>;
pub type VariableId = Id<'V'>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DVariable(pub VariableId);

impl DVariable {
    pub fn new(id: VariableId) -> Self {
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
    pub(crate) fn get_invariants(&self) -> &[ItemPtr] {
        &self.invariants[..]
    }

    pub(crate) fn get_dependencies(&self) -> &[ItemPtr] {
        &self.dependencies
    }

    pub(crate) fn get_var_dependencies(&self, env: &mut Environment) -> Dependencies {
        let mut result = Dependencies::new();
        for &dep in &self.dependencies {
            result.append(env.get_dependencies(dep));
        }
        result
    }

    pub fn assignment_justifications(
        &self,
        value: ItemPtr,
        env: &mut Environment,
        other_subs: &Substitutions,
        limit: u32,
    ) -> Vec<ItemPtr> {
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

impl_any_eq_from_regular_eq!(DVariable);

impl ItemDefinition for DVariable {
    fn dyn_clone(&self) -> Box<dyn ItemDefinition> {
        Box::new(self.clone())
    }
}

impl InvariantsFeature for DVariable {
    fn get_invariants_using_context(
        &self,
        this: &ItemPtr,
        ctx: &mut Icc,
        _: OnlyCalledByIcc,
    ) -> InvariantsResult {
        let statements = self.0.invariants.clone();
        let dependencies = hashset![this];
        todo!()
        // env.push_invariant_set(InvariantSet::new_statements_depending_on(
        //     this,
        //     statements,
        //     dependencies,
        // ))
    }
}

impl DependenciesFeature for DVariable {
    fn get_dependencies_using_context(&self, ctx: &mut Dcc, _: OnlyCalledByDcc) -> DepResult {
        let mut deps = Dependencies::new();
        for dep in ctx.get_variable(self.0).dependencies.clone() {
            deps.append(ctx.get_dependencies(dep));
        }
        deps.push_eager(ctx.get_variable(self.0).clone().as_dependency());
        for inv in ctx.get_variable(self.0).invariants.clone() {
            deps.append(ctx.get_dependencies(inv));
        }
        deps
    }
}

impl EqualityFeature for DVariable {
    fn get_equality_using_context(&self, ctx: &Ecc) -> EqualResult {
        unreachable!()
    }
}

#[derive(Debug, Clone)]
pub struct SVariableInvariants(pub ItemPtr);

impl Scope for SVariableInvariants {
    fn dyn_clone(&self) -> Box<dyn Scope> {
        Box::new(self.clone())
    }

    fn local_lookup_ident(&self, _env: &mut Environment, ident: &str) -> LookupIdentResult {
        Ok(if ident == "SELF" { Some(self.0) } else { None })
    }

    fn local_reverse_lookup_ident<'a, 'x>(
        &self,
        _env: &'a mut Environment,
        value: ItemPtr,
    ) -> ReverseLookupIdentResult {
        Ok(if value == self.0 {
            Some("SELF".to_owned())
        } else {
            None
        })
    }

    fn local_get_invariant_sets(&self, _env: &mut Environment) -> Vec<InvariantSetPtr> {
        vec![]
    }

    fn parent(&self) -> Option<ItemPtr> {
        Some(self.0)
    }
}
