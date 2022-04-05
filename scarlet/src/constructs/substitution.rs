use super::{variable::VariableId, Construct, ItemId};
use crate::{
    environment::{
        dependencies::{DepResult, Dependencies},
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
}

impl CSubstitution {
    pub fn new<'x>(base: ItemId, subs: Substitutions) -> Self {
        Self { base, subs }
    }

    pub fn new_unchecked(base: ItemId, subs: Substitutions) -> Self {
        Self::new(base, subs)
    }

    pub fn base(&self) -> ItemId {
        self.base
    }

    pub fn substitutions(&self) -> &Substitutions {
        &self.subs
    }

    pub fn sub_deps(base: Dependencies, subs: &Substitutions, env: &mut Environment) -> DepResult {
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
        deps
    }
}

impl_any_eq_for_construct!(CSubstitution);

impl Construct for CSubstitution {
    fn dyn_clone(&self) -> Box<dyn Construct> {
        Box::new(self.clone())
    }

    fn contents<'x>(&self) -> Vec<ItemId> {
        vec![self.base]
            .into_iter()
            .chain(self.subs.iter().map(|x| x.1))
            .collect()
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
        Self::sub_deps(base, &self.subs, env)
    }

    fn dereference(
        &self,
        _env: &mut Environment,
    ) -> Option<(ItemId, Option<&Substitutions>, Option<Vec<VariableId>>)> {
        Some((self.base, Some(&self.subs), None))
    }
}
