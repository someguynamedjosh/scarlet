use std::collections::HashSet;

use crate::{
    environment::Environment,
    impl_any_eq_from_regular_eq,
    item::{
        check::CheckFeature,
        definitions::variable::{DVariable, VariableId},
        dependencies::{Dcc, DepResult, Dependencies, DependenciesFeature, OnlyCalledByDcc},
        equality::{EqualResult, EqualityFeature},
        invariants::{
            Icc, InvariantSet, InvariantSetPtr, InvariantsFeature, InvariantsResult,
            OnlyCalledByIcc,
        },
        ItemDefinition, ItemPtr,
    },
    shared::OrderedMap,
};

pub type Substitutions = OrderedMap<VariableId, ItemPtr>;

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

    pub fn base(&self) -> ItemPtr {
        self.base
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
            if let Some((_, rep)) = subs.iter().find(|(var, _)| *var == dep.id) {
                let replaced_deps = rep.dependencies();
                let replaced_err = replaced_deps.error();
                for rdep in replaced_deps.into_variables() {
                    if !dep.swallow.contains(&rdep.id) {
                        deps.push_eager(rdep);
                    }
                }
                if let Some(err) = replaced_err {
                    deps.append(Dependencies::new_error(err));
                }
            } else {
                deps.push_eager(dep.clone());
            }
        }
        if let Some(err) = base_error {
            deps.append(Dependencies::new_error(err));
        }
        for &dep in justifications {
            if let Ok(Some(var)) = dep.downcast::<DVariable>() {
                let id = var.get_id();
                deps.push_eager(id.ptr_clone().as_dependency());
            } else {
                deps.append(dep.dependencies());
            }
        }
        deps
    }
}

impl_any_eq_from_regular_eq!(DSubstitution);

impl ItemDefinition for DSubstitution {
    fn dyn_clone(&self) -> Box<dyn ItemDefinition> {
        Box::new(self.clone())
    }

    fn contents(&self) -> Vec<ItemPtr> {
        vec![self.base]
            .into_iter()
            .chain(self.subs.iter().map(|x| x.1))
            .collect()
    }
}

impl CheckFeature for DSubstitution {}

impl DependenciesFeature for DSubstitution {
    fn get_dependencies_using_context(&self, ctx: &mut Dcc, _: OnlyCalledByDcc) -> DepResult {
        let base = ctx.get_dependencies(&self.base);
        let invs = self.invs.dependencies().clone();
        Self::sub_deps(base, &self.subs, &invs)
    }
}

impl EqualityFeature for DSubstitution {
    fn get_equality_using_context(&self, ctx: &crate::item::equality::Ecc) -> EqualResult {
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
        Ok(self.invs)
    }
}
