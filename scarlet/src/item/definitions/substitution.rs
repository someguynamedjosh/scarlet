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
        equality::{Ecc, Equal, EqualResult, EqualSuccess, EqualityFeature, OnlyCalledByEcc},
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
                        deps.push_eager(rdep);
                    }
                }
                if let Some(err) = replaced_err {
                    deps.append(Dependencies::new_error(err.clone()));
                }
            } else {
                if TRACE {
                    println!("UNCHANGED {:#?}", dep);
                }
                deps.push_eager(dep.clone());
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

    fn contents(&self) -> Vec<(ContainmentType, &ItemPtr)> {
        vec![(ContainmentType::Computational, &self.base)]
            .into_iter()
            .chain(
                self.subs
                    .iter()
                    .map(|x| (ContainmentType::Computational, &x.1)),
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
        let base_eq = ctx
            .with_primary(self.base.ptr_clone())
            .get_equality_left()?;
        let equal = if let Equal::Yes(original_subs) = base_eq {
            let mut valid_subs = Vec::new();
            for (left_subs, right_subs) in original_subs {
                let (mut primary_subs, mut other_subs) =
                    if ctx.currently_computing_equality_for_lhs() {
                        (left_subs, right_subs)
                    } else {
                        (right_subs, left_subs)
                    };
                // Contains tuples of what the base has and what our substitution replaces it
                // with.
                let mut subs_to_check = Vec::new();
                'next_self_sub: for (target, value) in &self.subs {
                    for dep in self.base.get_dependencies().into_variables() {
                        if dep.var.is_same_instance_as(target) && !dep.affects_return_value {
                            continue 'next_self_sub;
                        }
                    }
                    let value = value.ptr_clone();
                    if let Some(original_value) = primary_subs.remove(target) {
                        subs_to_check.push((original_value.1, value));
                    } else {
                        subs_to_check.push((target.borrow().item().ptr_clone(), value));
                    }
                }
                for (target, value) in other_subs.iter_mut() {
                    let deps = value.get_dependencies();
                    let mut subs = Substitutions::new();
                    'next_value_dependency: for dep in deps.into_variables() {
                        if !dep.affects_return_value {
                            continue 'next_value_dependency;
                        }
                        if let Some(replacement) = self.subs.get(&dep.var) {
                            for target_dep in target.borrow().dependencies() {
                                if target_dep
                                    .dereference()
                                    .is_same_instance_as(&dep.var.borrow().item().dereference())
                                {
                                    continue 'next_value_dependency;
                                }
                            }
                            subs.insert_no_replace(dep.var, replacement.ptr_clone());
                        }
                    }
                    *value = unchecked_substitution(value.ptr_clone(), &subs);
                }
                let mut result = if ctx.currently_computing_equality_for_lhs() {
                    Equal::yes1(primary_subs, other_subs)
                } else {
                    Equal::yes1(other_subs, primary_subs)
                };
                for (original_value, replaced_value) in subs_to_check {
                    let original_is_replaced = if ctx.currently_computing_equality_for_lhs() {
                        replaced_value.get_trimmed_equality(&original_value)?
                    } else {
                        original_value.get_trimmed_equality(&replaced_value)?
                    };
                    result = Equal::and(vec![result, original_is_replaced]);
                }
                result.filter(&*ctx);
                if let Equal::Yes(mut result) = result {
                    valid_subs.append(&mut result);
                }
            }
            if valid_subs.len() > 0 {
                Equal::Yes(valid_subs)
            } else {
                Equal::Unknown
            }
        } else {
            Equal::Unknown
        };
        Ok(EqualSuccess {
            equal,
            unique: true,
        })
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
