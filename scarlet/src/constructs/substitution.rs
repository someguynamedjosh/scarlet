use super::{
    variable::{CVariable, VariableId},
    Construct, ItemId, GenInvResult,
};
use crate::{
    environment::{
        dependencies::{DepResult, Dependencies},
        discover_equality::{DeqPriority, DeqResult, DeqSide, Equal},
        invariants::Invariant,
        CheckResult, Environment,
    },
    impl_any_eq_for_construct,
    scope::Scope,
    shared::OrderedMap,
};

pub type Substitutions = OrderedMap<VariableId, ItemId>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CSubstitution {
    base: ItemId,
    subs: Substitutions,
    invs: Vec<crate::environment::invariants::Invariant>,
}

impl CSubstitution {
    pub fn new<'x>(
        base: ItemId,
        subs: Substitutions,
        invs: Vec<crate::environment::invariants::Invariant>,
    ) -> Self {
        Self { base, subs, invs }
    }

    pub fn new_unchecked(base: ItemId, subs: Substitutions) -> Self {
        Self::new(base, subs, Vec::new())
    }

    pub fn base(&self) -> ItemId {
        self.base
    }

    pub fn substitutions(&self) -> &Substitutions {
        &self.subs
    }

    pub fn sub_deps(
        base: Dependencies,
        subs: &Substitutions,
        invs: &[Invariant],
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
        for inv in invs.iter() {
            for &dep in &inv.dependencies {
                if let Ok(Some(var)) = env.get_and_downcast_construct_definition::<CVariable>(dep) {
                    let id = var.get_id();
                    deps.push_eager(env.get_variable(id).clone().as_dependency(env));
                } else {
                    deps.append(env.get_dependencies(dep));
                }
            }
        }
        deps
    }
}

impl_any_eq_for_construct!(CSubstitution);

impl Construct for CSubstitution {
    fn dyn_clone(&self) -> Box<dyn Construct> {
        Box::new(self.clone())
    }

    fn check<'x>(
        &self,
        _env: &mut Environment<'x>,
        _this: ItemId,
        _scope: Box<dyn Scope>,
    ) -> CheckResult {
        Ok(())
    }

    fn get_dependencies<'x>(&self, env: &mut Environment<'x>) -> DepResult {
        let base = env.get_dependencies(self.base);
        Self::sub_deps(base, &self.subs, &self.invs[..], env)
    }

    fn generated_invariants<'x>(
        &self,
        _this: ItemId,
        _env: &mut Environment<'x>,
    ) -> GenInvResult {
        self.invs.clone()
    }

    fn dereference(&self) -> Option<(ItemId, Option<&Substitutions>)> {
        Some((self.base, Some(&self.subs)))
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
        // Special behavior is hard-coded into Environment::discover_equality_with_subs.
        unreachable!()
    }
}
