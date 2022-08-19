use std::{collections::HashSet, fmt::Debug};

use crate::{
    impl_any_eq_from_regular_eq,
    item::{
        check::CheckFeature,
        definitions::variable::{DVariable, VariablePtr},
        dependencies::{
            Dcc, DepResult, Dependencies, DependenciesFeature, DependencyCalculationContext,
            OnlyCalledByDcc,
        },
        equality::{Ecc, Equal, EqualResult, EqualityFeature, OnlyCalledByEcc},
        invariants::{
            Icc, InvariantSet, InvariantSetPtr, InvariantsFeature, InvariantsResult,
            OnlyCalledByIcc,
        },
        util::unchecked_substitution,
        ContainmentType, ItemDefinition, ItemPtr,
    },
    shared::OrderedMap,
    util::PtrExtension,
};

pub type Substitutions = OrderedMap<VariablePtr, ItemPtr>;

#[derive(Clone, PartialEq, Eq)]
pub struct DSubstitution {
    base: ItemPtr,
    subs: Substitutions,
    invs: InvariantSetPtr,
}

impl Debug for DSubstitution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DSubstitution")
            .field("base", &self.base)
            .field("subs", &self.subs)
            .field("invs", &self.invs)
            .finish()
    }
}

impl DSubstitution {
    pub fn new(base: ItemPtr, subs: Substitutions, invs: InvariantSetPtr) -> Self {
        Self { base, subs, invs }
    }

    pub fn new_unchecked(base: ItemPtr, subs: Substitutions) -> Self {
        Self::new(base.ptr_clone(), subs, InvariantSet::new_empty(base))
    }

    pub fn base(&self) -> &ItemPtr {
        &self.base
    }

    // Only allows access if self is an *unchecked* substitution. This ensures
    // soundness.
    pub fn base_mut(&mut self) -> Option<&mut ItemPtr> {
        if self.invs.borrow().justification_requirements().len() == 0 {
            Some(&mut self.base)
        } else {
            None
        }
    }

    pub fn substitutions(&self) -> &Substitutions {
        &self.subs
    }

    // Only allows access if self is an *unchecked* substitution. This ensures
    // soundness.
    pub fn substitutions_mut(&mut self) -> Option<&mut Substitutions> {
        if self.invs.borrow().justification_requirements().len() == 0 {
            Some(&mut self.subs)
        } else {
            None
        }
    }

    pub fn sub_deps(
        ctx: &mut DependencyCalculationContext,
        base: Dependencies,
        subs: &Substitutions,
        justifications: &HashSet<ItemPtr>,
        affects_return_value: bool,
    ) -> DepResult {
        const TRACE: bool = false;
        if TRACE {
            println!(
                "--------------------------------------------------------------------------------"
            );
        }
        let mut deps = Dependencies::new();
        let base_error = base.error();
        for dep in base.as_variables() {
            if TRACE {
                println!("-");
            }
            if let Some((_, rep)) = subs
                .iter()
                .find(|(var, _)| var.is_same_instance_as(&dep.var))
            {
                let replaced_deps =
                    ctx.get_dependencies(rep, affects_return_value && dep.affects_return_value);
                let replaced_err = replaced_deps.error().clone();
                for rdep in replaced_deps.into_variables() {
                    if !dep.swallow.contains(&rdep.var) {
                        if TRACE {
                            println!("{:#?}", rdep);
                        }
                        deps.push_value(rdep);
                    }
                }
                if let Some(err) = replaced_err {
                    deps.append(Dependencies::new_error(err.clone()));
                }
            } else {
                if TRACE {
                    println!("UNCHANGED {:#?}", dep);
                }
                deps.push_value(dep.clone());
            }
        }
        if let Some(err) = base_error {
            deps.append(Dependencies::new_error(err.clone()));
        }
        for dep in justifications {
            if let Some(var) = dep.downcast_definition::<DVariable>() {
                deps.append(var.as_dependency(false));
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

    fn contents(&self) -> Vec<(ContainmentType, ItemPtr)> {
        vec![(ContainmentType::Computational, self.base.ptr_clone())]
            .into_iter()
            .chain(
                self.subs
                    .iter()
                    .map(|x| (ContainmentType::Computational, x.1.ptr_clone())),
            )
            .collect()
    }
}

impl CheckFeature for DSubstitution {}

impl DependenciesFeature for DSubstitution {
    fn get_dependencies_using_context(
        &self,
        _this: &ItemPtr,
        ctx: &mut Dcc,
        affects_return_value: bool,
        _: OnlyCalledByDcc,
    ) -> DepResult {
        let base = ctx.get_dependencies(&self.base, affects_return_value);
        let invs = self.invs.borrow().dependencies().clone();
        Self::sub_deps(ctx, base, &self.subs, &invs, affects_return_value)
    }
}

impl EqualityFeature for DSubstitution {
    fn get_equality_using_context(&self, ctx: &mut Ecc, _: OnlyCalledByEcc) -> EqualResult {
        ctx.prepend_substitutions_for_primary(self.subs.clone());
        ctx.with_primary(self.base.ptr_clone()).get_equality_left()
    }
}

impl InvariantsFeature for DSubstitution {
    fn get_invariants_using_context(
        &self,
        _this: &ItemPtr,
        _ctx: &mut Icc,
        _: OnlyCalledByIcc,
    ) -> InvariantsResult {
        Ok(self.invs.ptr_clone())
    }
}
