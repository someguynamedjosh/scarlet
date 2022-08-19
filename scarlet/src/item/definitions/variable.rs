use std::{
    cell::RefCell,
    fmt::{self, Debug, Formatter},
    rc::Rc,
};

use itertools::Itertools;
use maplit::hashset;

use crate::{
    environment::Environment,
    impl_any_eq_from_regular_eq,
    item::{
        check::CheckFeature,
        definitions::substitution::Substitutions,
        dependencies::{
            Dcc, DepResult, Dependencies, DependenciesFeature, Dependency, OnlyCalledByDcc,
            Requirement,
        },
        equality::{Ecc, Equal, EqualResult, EqualityFeature, OnlyCalledByEcc},
        invariants::{
            Icc, InvariantSet, InvariantSetPtr, InvariantsFeature, InvariantsResult,
            JustificationRequirement, OnlyCalledByIcc,
        },
        util::unchecked_substitution,
        ContainmentType, Item, ItemDefinition, ItemPtr,
    },
    scope::{LookupIdentResult, ReverseLookupIdentResult, Scope},
    util::{rcrc, PtrExtension},
};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct VariableOrder {
    /// Explicitly defined order, 0-255.
    pub major_order: u8,
    /// Implicit order by which file it's in.
    file_order: u32,
    /// Implicit order by position in file.
    minor_order: u32,
}

impl Debug for VariableOrder {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}:{}",
            self.major_order, self.file_order, self.minor_order
        )
    }
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
pub enum VariableKind {
    Theorem(ItemPtr),
    Value,
}

impl VariableKind {
    pub fn as_theorem(&self) -> Option<&ItemPtr> {
        if let Self::Theorem(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Variable {
    item: ItemPtr,
    kind: VariableKind,
    dependencies: Vec<ItemPtr>,
    order: VariableOrder,
}

impl Debug for Variable {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut base = f.debug_struct("Variable");
        base.field("item", &self.item.debug_label());
        base.field("kind", &self.kind);
        if self.dependencies.len() > 0 {
            base.field("dependencies", &self.dependencies);
        }
        base.field("order", &self.order).finish()
    }
}

pub type VariablePtr = Rc<RefCell<Variable>>;

impl Variable {
    pub fn item(&self) -> &ItemPtr {
        &self.item
    }

    pub fn required_theorem(&self) -> Option<&ItemPtr> {
        self.kind.as_theorem()
    }

    pub fn dependencies(&self) -> &[ItemPtr] {
        &self.dependencies[..]
    }

    pub fn order(&self) -> &VariableOrder {
        &self.order
    }

    pub fn new_theorem(
        statement: ItemPtr,
        dependencies: Vec<ItemPtr>,
        item: ItemPtr,
        order: VariableOrder,
    ) -> Rc<RefCell<Variable>> {
        rcrc(Self {
            dependencies,
            kind: VariableKind::Theorem(statement),
            item,
            order,
        })
    }

    pub fn new_value(
        dependencies: Vec<ItemPtr>,
        item: ItemPtr,
        order: VariableOrder,
    ) -> Rc<RefCell<Variable>> {
        rcrc(Self {
            dependencies,
            kind: VariableKind::Value,
            item,
            order,
        })
    }
}

#[derive(Clone)]
pub struct DVariable(VariablePtr, Vec<ItemPtr>);

impl Debug for DVariable {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.borrow().fmt(f)
    }
}

impl PartialEq for DVariable {
    fn eq(&self, other: &Self) -> bool {
        self.0.is_same_instance_as(&other.0)
    }
}

impl DVariable {
    pub fn get_variable(&self) -> &VariablePtr {
        &self.0
    }

    pub fn as_dependency(&self, affects_return_value: bool) -> Dependencies {
        Variable::as_dependency(&self.0, affects_return_value)
    }

    pub fn new_theorem(
        statement: ItemPtr,
        dependencies: Vec<ItemPtr>,
        order: VariableOrder,
        scope: Box<dyn Scope>,
    ) -> ItemPtr {
        let placeholder = Item::placeholder(format!("variable item"));
        let variable = Variable::new_theorem(
            statement.ptr_clone(),
            dependencies.clone(),
            placeholder,
            order,
        );
        let def = DVariable(
            variable,
            dependencies
                .into_iter()
                .chain(std::iter::once(statement))
                .collect_vec(),
        );
        Item::new_self_referencing(def, scope, |ptr, this| {
            this.0.borrow_mut().item = ptr;
        })
    }

    pub fn new_value(
        dependencies: Vec<ItemPtr>,
        order: VariableOrder,
        scope: Box<dyn Scope>,
    ) -> ItemPtr {
        let placeholder = Item::placeholder(format!("variable item"));
        let variable = Variable::new_value(dependencies.clone(), placeholder, order);
        let def = DVariable(variable, dependencies);
        Item::new_self_referencing(def, scope, |ptr, this| {
            this.0.borrow_mut().item = ptr;
        })
    }
}

impl Variable {
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

    pub fn as_dependency(this: &VariablePtr, affects_return_value: bool) -> Dependencies {
        let mut deps = Dependencies::new();
        for dep in &this.borrow().dependencies {
            deps.append(dep.get_dependencies());
        }
        let result = match deps.as_complete_variables() {
            Ok(swallowed) => {
                let mut res = Dependencies::new();

                if let Some(theorem) = this.borrow().kind.as_theorem() {
                    res.push_requirement(Requirement {
                        statement: theorem.ptr_clone(),
                        swallow_dependencies: swallowed
                            .filter_map(|x| x.var.borrow().kind.as_theorem().map(|x| x.ptr_clone()))
                            .collect(),
                        order: this.borrow().order.clone(),
                    });
                } else {
                    res.push_value(Dependency {
                        var: this.ptr_clone(),
                        swallow: swallowed.map(|x| x.var.ptr_clone()).collect(),
                        order: this.borrow().order.clone(),
                        affects_return_value,
                    });
                }
                res
            }
            Err(err) => Dependencies::new_error(err.clone()),
        };
        drop(deps);
        result
    }
}

impl_any_eq_from_regular_eq!(DVariable);

impl ItemDefinition for DVariable {
    fn clone_into_box(&self) -> Box<dyn ItemDefinition> {
        Box::new(self.clone())
    }

    fn contents(&self) -> Vec<(ContainmentType, ItemPtr)> {
        self.1
            .iter()
            .map(|inv_or_dep| (ContainmentType::Definitional, inv_or_dep.ptr_clone()))
            .collect_vec()
    }
}

impl CheckFeature for DVariable {}

impl DependenciesFeature for DVariable {
    fn get_dependencies_using_context(
        &self,
        _this: &ItemPtr,
        ctx: &mut Dcc,
        affects_return_value: bool,
        _: OnlyCalledByDcc,
    ) -> DepResult {
        let mut deps = Dependencies::new();
        for dep in self.0.borrow().dependencies.clone() {
            deps.append(ctx.get_dependencies(&dep, affects_return_value));
        }
        deps.append(Variable::as_dependency(&self.0, affects_return_value));
        if let Some(statement) = self.0.borrow().required_theorem() {
            deps.append(ctx.get_dependencies(statement, false));
        }
        deps
    }
}

impl EqualityFeature for DVariable {
    fn get_equality_using_context(&self, ctx: &mut Ecc, _: OnlyCalledByEcc) -> EqualResult {
        if let Some(other_var) = ctx.other().downcast_resolved_definition::<Self>()? {
            if other_var.0.is_same_instance_as(&self.0) && ctx.no_subs() {
                return Ok(Equal::yes());
            } else if let Ok(Some(mut ctx)) =
                ctx.try_select_value_substituted_for_var_in_other(&other_var.0)
            {
                drop(other_var);
                return ctx.get_equality_left();
            }
        }
        if let Ok(Some(mut ctx)) = ctx.try_select_value_substituted_for_var_in_primary(&self.0) {
            return ctx.get_equality_left();
        }
        let num_deps = self.0.borrow().dependencies.len();
        if num_deps == 0 {
            let value = ctx.other_with_subs().ptr_clone();
            let subs: Substitutions = vec![(self.0.ptr_clone(), value)].into_iter().collect();
            if ctx.currently_computing_equality_for_lhs() {
                Ok(Equal::Yes(subs, Substitutions::new()))
            } else {
                Ok(Equal::Yes(Substitutions::new(), subs))
            }
        } else {
            let mut acceptable_dependencies = Vec::new();
            for dep in ctx.other().get_dependencies().into_variables() {
                if dep.swallow.len() > 0 {
                    continue;
                }
                if !dep.affects_return_value {
                    continue;
                }
                acceptable_dependencies.push(dep);
            }
            if acceptable_dependencies.len() >= num_deps {
                let self_var = self.0.borrow();
                let self_deps = self_var
                    .dependencies
                    .iter()
                    .flat_map(|x| x.get_dependencies().into_variables())
                    .collect_vec();
                let pairings = self_deps
                    .into_iter()
                    .zip(acceptable_dependencies.into_iter());

                let mut results = vec![];
                let mut other_subs = Substitutions::new();
                for (self_dep, other_dep) in pairings {
                    let self_dep = self_dep.var;
                    if self_dep.borrow().dependencies.len() > 0 {
                        return Ok(Equal::Unknown);
                    }
                    let self_dep_item = self_dep.borrow().item.ptr_clone();
                    let other_dep = other_dep.var;
                    let other_dep_item = other_dep.borrow().item.ptr_clone();
                    results.push(
                        ctx.with_primary_and_other(
                            self_dep_item.ptr_clone(),
                            other_dep_item.ptr_clone(),
                        )
                        .get_equality_left()?,
                    );
                    other_subs.insert_no_replace(other_dep, self_dep_item);
                }

                let mut subbed_right = unchecked_substitution(ctx.other().ptr_clone(), &other_subs);
                let mut deps = ctx.other().get_dependencies();
                for (target, _) in &other_subs {
                    deps.remove(target);
                }
                for subs in ctx.other_subs() {
                    let mut new_subs = Substitutions::new();
                    let mut new_deps = Dependencies::new();
                    for (target, value) in subs {
                        if deps.contains_var(target) {
                            new_subs.insert_no_replace(target.ptr_clone(), value.ptr_clone());
                            deps.remove(target);
                            new_deps.append(value.get_dependencies());
                        }
                    }
                    if new_subs.len() > 0 {
                        subbed_right = unchecked_substitution(subbed_right, &new_subs);
                    }
                }
                if let Equal::Yes(mut lhs, mut rhs) = Equal::and(results) {
                    let primary_subs = if ctx.currently_computing_equality_for_lhs() {
                        &mut lhs
                    } else {
                        &mut rhs
                    };
                    primary_subs.insert_no_replace(self.0.ptr_clone(), subbed_right);
                    Ok(Equal::Yes(lhs, rhs))
                } else {
                    Ok(Equal::Unknown)
                }
            } else {
                Ok(Equal::Unknown)
            }
        }
    }
}

impl InvariantsFeature for DVariable {
    fn get_invariants_using_context(
        &self,
        this: &ItemPtr,
        _ctx: &mut Icc,
        _: OnlyCalledByIcc,
    ) -> InvariantsResult {
        let statements = self
            .0
            .borrow()
            .required_theorem()
            .into_iter()
            .map(ItemPtr::ptr_clone)
            .collect_vec();
        let dependencies = hashset![this.ptr_clone()];
        Ok(InvariantSet::new_root_statements_depending_on(
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

    fn local_lookup_ident(&self, ident: &str) -> LookupIdentResult {
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
        Ok(if self.0.dereference().is_same_instance_as(&value) {
            Some("SELF".to_owned())
        } else {
            None
        })
    }

    fn local_get_invariant_sets(&self) -> Vec<InvariantSetPtr> {
        vec![]
    }

    fn parent(&self) -> Option<ItemPtr> {
        Some(self.0.ptr_clone())
    }
}
