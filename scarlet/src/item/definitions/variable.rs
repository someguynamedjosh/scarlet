use std::{cell::RefCell, rc::Rc};

use maplit::hashset;

use crate::{
    environment::Environment,
    impl_any_eq_from_regular_eq,
    item::{
        check::CheckFeature,
        definitions::{decision::DDecision, substitution::Substitutions},
        dependencies::{
            Dcc, DepResult, Dependencies, DependenciesFeature, Dependency, OnlyCalledByDcc,
        },
        equality::{Ecc, Equal, EqualResult, EqualityFeature, OnlyCalledByEcc, PermissionToRefine},
        invariants::{
            self, Icc, InvariantSet, InvariantSetPtr, InvariantsFeature, InvariantsResult,
            OnlyCalledByIcc,
        },
        util::unchecked_substitution,
        Item, ItemDefinition, ItemPtr,
    },
    scope::{
        LookupIdentResult, LookupInvariantError, LookupInvariantResult, ReverseLookupIdentResult,
        SPlain, SRoot, Scope,
    },
    shared::{Id, Pool},
    util::{rcrc, PtrExtension},
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
    item: ItemPtr,
    invariants: Vec<ItemPtr>,
    dependencies: Vec<ItemPtr>,
    order: VariableOrder,
}

pub type VariablePtr = Rc<RefCell<Variable>>;

impl Variable {
    pub fn item(&self) -> &ItemPtr {
        &self.item
    }

    pub fn invariants(&self) -> &[ItemPtr] {
        &self.invariants[..]
    }

    pub fn dependencies(&self) -> &[ItemPtr] {
        &self.dependencies[..]
    }

    pub fn order(&self) -> &VariableOrder {
        &self.order
    }
}

#[derive(Clone, Debug)]
pub struct DVariable(VariablePtr);

impl PartialEq for DVariable {
    fn eq(&self, other: &Self) -> bool {
        self.0.is_same_instance_as(&other.0)
    }
}

impl DVariable {
    pub fn get_variable(&self) -> &VariablePtr {
        &self.0
    }

    pub fn as_dependency(&self) -> Dependency {
        Variable::as_dependency(&self.0)
    }

    pub fn new(
        invariants: Vec<ItemPtr>,
        dependencies: Vec<ItemPtr>,
        order: VariableOrder,
        scope: Box<dyn Scope>,
    ) -> ItemPtr {
        let placeholder = Item::placeholder();
        let variable = Variable {
            item: placeholder,
            invariants,
            dependencies,
            order,
        };
        let def = DVariable(rcrc(variable));
        Item::new_self_referencing(def, scope, |ptr, this| {
            this.0.borrow_mut().item = ptr;
        })
    }
}

impl Variable {
    pub(crate) fn get_invariants(&self) -> &[ItemPtr] {
        &self.invariants[..]
    }

    pub(crate) fn get_dependencies(&self) -> &[ItemPtr] {
        &self.dependencies
    }

    pub(crate) fn get_var_dependencies(&self) -> Dependencies {
        let mut result = Dependencies::new();
        for dep in &self.dependencies {
            result.append(dep.get_dependencies());
        }
        result
    }

    pub fn assignment_justifications(
        this: &VariablePtr,
        value: ItemPtr,
        other_subs: &Substitutions,
    ) -> Vec<ItemPtr> {
        let mut substitutions = other_subs.clone();
        let mut justifications = Vec::new();
        substitutions.insert_no_replace(this.ptr_clone(), value);
        for inv in &this.borrow().invariants {
            let subbed = unchecked_substitution(inv.ptr_clone(), &substitutions);
            justifications.push(subbed);
        }
        justifications
    }

    pub fn as_dependency(this: &VariablePtr) -> Dependency {
        let mut deps = Dependencies::new();
        for dep in &this.borrow().dependencies {
            deps.append(dep.get_dependencies());
        }
        Dependency {
            var: this.ptr_clone(),
            swallow: deps.as_variables().map(|x| x.var.ptr_clone()).collect(),
            order: this.borrow().order.clone(),
        }
    }
}

impl_any_eq_from_regular_eq!(DVariable);

impl ItemDefinition for DVariable {
    fn clone_into_box(&self) -> Box<dyn ItemDefinition> {
        Box::new(self.clone())
    }
}

impl CheckFeature for DVariable {}

impl DependenciesFeature for DVariable {
    fn get_dependencies_using_context(&self, ctx: &mut Dcc, _: OnlyCalledByDcc) -> DepResult {
        let mut deps = Dependencies::new();
        for dep in self.0.borrow().dependencies.clone() {
            deps.append(dep.get_dependencies());
        }
        deps.push_eager(Variable::as_dependency(&self.0));
        for inv in &self.0.borrow().invariants {
            deps.append(inv.get_dependencies());
        }
        deps
    }
}

impl EqualityFeature for DVariable {
    fn get_equality_using_context(
        &self,
        ctx: &Ecc,
        can_refine: PermissionToRefine,
        _: OnlyCalledByEcc,
    ) -> EqualResult {
        unreachable!()
    }
}

impl InvariantsFeature for DVariable {
    fn get_invariants_using_context(
        &self,
        this: &ItemPtr,
        ctx: &mut Icc,
        _: OnlyCalledByIcc,
    ) -> InvariantsResult {
        let statements = self.0.borrow().invariants.clone();
        let dependencies = hashset![this.ptr_clone()];
        Ok(InvariantSet::new_statements_depending_on(
            this.ptr_clone(),
            statements,
            dependencies,
        ))
    }
}

#[derive(Debug, Clone)]
pub struct SVariableInvariants(pub ItemPtr);

impl Scope for SVariableInvariants {
    fn dyn_clone(&self) -> Box<dyn Scope> {
        Box::new(self.clone())
    }

    fn local_lookup_ident(&self, _env: &mut Environment, ident: &str) -> LookupIdentResult {
        Ok(if ident == "SELF" {
            Some(self.0.ptr_clone())
        } else {
            None
        })
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
        Some(self.0.ptr_clone())
    }
}
