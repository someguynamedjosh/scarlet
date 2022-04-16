use std::collections::HashSet;

use crate::item::{
    variable::{CVariable, VariableId},
    ItemDefinition, GenInvResult, ItemPtr,
};
use crate::{
    environment::{
        dependencies::{DepResult, Dependencies},
        discover_equality::{DeqPriority, DeqResult, DeqSide, Equal},
        invariants::{InvariantSet, InvariantSetPtr},
        CheckResult, Environment,
    },
    impl_any_eq_for_construct,
    scope::Scope,
    shared::OrderedMap,
};

pub type Substitutions = OrderedMap<VariableId, ItemPtr>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CSubstitution {
    base: ItemPtr,
    subs: Substitutions,
    invs: InvariantSetPtr,
}

impl CSubstitution {
    pub fn new(base: ItemPtr, subs: Substitutions, invs: InvariantSetPtr) -> Self {
        Self { base, subs, invs }
    }

    pub fn new_unchecked(
        env: &mut Environment,
        this: ItemPtr,
        base: ItemPtr,
        subs: Substitutions,
    ) -> Self {
        let base_deps = env.get_dependencies(base);
        Self::new(
            base,
            subs,
            env.push_invariant_set(InvariantSet::new_empty(this)),
        )
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
        env: &mut Environment,
    ) -> DepResult {
        let mut deps = Dependencies::new();
        let base_error = base.error();
        for dep in base.as_variables() {
            if let Some((_, rep)) = subs.iter().find(|(var, _)| *var == dep.id) {
                let replaced_deps = env.get_dependencies(*rep);
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
            if let Ok(Some(var)) = env.get_and_downcast_construct_definition::<CVariable>(dep) {
                let id = var.get_id();
                deps.push_eager(env.get_variable(id).clone().as_dependency(env));
            } else {
                deps.append(env.get_dependencies(dep));
            }
        }
        deps
    }
}

impl_any_eq_for_construct!(CSubstitution);

impl ItemDefinition for CSubstitution {
    fn dyn_clone(&self) -> Box<dyn ItemDefinition> {
        Box::new(self.clone())
    }

    fn contents(&self) -> Vec<ItemPtr> {
        vec![self.base]
            .into_iter()
            .chain(self.subs.iter().map(|x| x.1))
            .collect()
    }

    fn get_dependencies(&self, env: &mut Environment) -> DepResult {
        let base = env.get_dependencies(self.base);
        let invs = env
            .get_invariant_set(self.invs)
            .clone()
            .dependencies()
            .clone();
        Self::sub_deps(base, &self.subs, &invs, env)
    }

    fn generated_invariants(&self, _this: ItemPtr, _env: &mut Environment) -> GenInvResult {
        self.invs
    }

    fn dereference(
        &self,
        env: &mut Environment,
    ) -> Option<(ItemPtr, Option<&Substitutions>, Option<Vec<ItemPtr>>)> {
        Some((self.base, Some(&self.subs), None))
    }

    fn discover_equality(
        &self,
        env: &mut Environment,
        self_subs: Vec<&Substitutions>,
        other_id: ItemPtr,
        other: &dyn ItemDefinition,
        other_subs: Vec<&Substitutions>,
        limit: u32,
    ) -> DeqResult {
        // Special behavior is hard-coded into Environment::discover_equality_with_subs.
        unreachable!()
    }
}
