use std::collections::HashSet;

use crate::{
    environment::Environment,
    impl_any_eq_from_regular_eq,
    item::{
        check::CheckFeature,
        definitions::variable::{DVariable, VariablePtr},
        dependencies::{Dcc, DepResult, Dependencies, DependenciesFeature, OnlyCalledByDcc},
        equality::{Ecc, EqualResult, EqualityFeature, OnlyCalledByEcc, PermissionToRefine},
        invariants::{
            Icc, InvariantSet, InvariantSetPtr, InvariantsFeature, InvariantsResult,
            OnlyCalledByIcc,
        },
        ItemDefinition, ItemPtr,
    },
    shared::OrderedMap,
    util::PtrExtension,
};

pub type Substitutions = OrderedMap<VariablePtr, ItemPtr>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DSubstitution {
    base: ItemPtr,
    subs: Substitutions,
    invs: InvariantSetPtr,
}

impl DSubstitution {
    pub fn new(base: ItemPtr, subs: Substitutions, invs: InvariantSetPtr) -> Self {
        Self { base, subs, invs }
    }

    pub fn new_unchecked(this: ItemPtr, base: ItemPtr, subs: Substitutions) -> Self {
        Self::new(base, subs, InvariantSet::new_empty(this))
    }

    pub fn base(&self) -> &ItemPtr {
        &self.base
    }

    pub fn substitutions(&self) -> &Substitutions {
        &self.subs
    }

    pub fn sub_deps(
        base: Dependencies,
        subs: &Substitutions,
        justifications: &HashSet<ItemPtr>,
    ) -> DepResult {
        let mut deps = Dependencies::new();
        let base_error = base.error();
        for dep in base.as_variables() {
            if let Some((_, rep)) = subs.iter().find(|(var, _)| *var == dep.var) {
                let replaced_deps = rep.get_dependencies();
                let replaced_err = replaced_deps.error().clone();
                for rdep in replaced_deps.into_variables() {
                    if !dep.swallow.contains(&rdep.var) {
                        deps.push_eager(rdep);
                    }
                }
                if let Some(err) = replaced_err {
                    deps.append(Dependencies::new_error(err.clone()));
                }
            } else {
                deps.push_eager(dep.clone());
            }
        }
        if let Some(err) = base_error {
            deps.append(Dependencies::new_error(err.clone()));
        }
        for dep in justifications {
            if let Some(var) = dep.downcast_definition::<DVariable>() {
                deps.push_eager(var.as_dependency());
            } else {
                deps.append(dep.get_dependencies());
            }
        }
        deps
    }
}

impl_any_eq_from_regular_eq!(DSubstitution);

impl ItemDefinition for DSubstitution {
    fn clone_into_box(&self) -> Box<dyn ItemDefinition> {
        Box::new(self.clone())
    }

    fn contents(&self) -> Vec<&ItemPtr> {
        vec![&self.base]
            .into_iter()
            .chain(self.subs.iter().map(|x| &x.1))
            .collect()
    }
}

impl CheckFeature for DSubstitution {}

impl DependenciesFeature for DSubstitution {
    fn get_dependencies_using_context(&self, ctx: &mut Dcc, _: OnlyCalledByDcc) -> DepResult {
        let base = ctx.get_dependencies(&self.base);
        let invs = self.invs.borrow().dependencies().clone();
        Self::sub_deps(base, &self.subs, &invs)
    }
}

impl EqualityFeature for DSubstitution {
    fn get_equality_using_context(
        &self,
        ctx: &Ecc,
        can_refine: PermissionToRefine,
        _: OnlyCalledByEcc,
    ) -> EqualResult {
        unreachable!()
    }
}

impl InvariantsFeature for DSubstitution {
    fn get_invariants_using_context(
        &self,
        this: &ItemPtr,
        ctx: &mut Icc,
        _: OnlyCalledByIcc,
    ) -> InvariantsResult {
        Ok(self.invs.ptr_clone())
    }
}
