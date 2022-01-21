use itertools::Itertools;

use super::{
    base::{Construct, ConstructId},
    downcast_construct,
    substitution::Substitutions,
    BoxedConstruct, ConstructDefinition, Invariant,
};
use crate::{
    environment::{dependencies::Dependencies, Environment},
    impl_any_eq_for_construct,
    shared::TripleBool,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CWithDependencies {
    base: ConstructId,
    dependencies: Vec<ConstructId>,
}

impl CWithDependencies {
    pub fn new<'x>(base: ConstructId, dependencies: Vec<ConstructId>) -> Self {
        Self { base, dependencies }
    }

    pub fn base(&self) -> ConstructId {
        self.base
    }

    pub(crate) fn dependencies(&self) -> &[ConstructId] {
        &self.dependencies
    }
}

impl_any_eq_for_construct!(CWithDependencies);

impl Construct for CWithDependencies {
    fn dyn_clone(&self) -> Box<dyn Construct> {
        Box::new(self.clone())
    }

    fn generated_invariants<'x>(
        &self,
        _this: ConstructId,
        env: &mut Environment<'x>,
    ) -> Vec<Invariant> {
        env.generated_invariants(self.base)
    }

    fn get_dependencies<'x>(&self, env: &mut Environment<'x>) -> Dependencies {
        let mut deps = Dependencies::new();
        let base_deps = env.get_dependencies(self.base);
        for &priority_dep in &self.dependencies {
            for dep in env.get_dependencies(priority_dep).into_variables() {
                if base_deps.contains(&dep) {
                    deps.push_eager(dep);
                }
            }
        }
        deps.append(base_deps);
        deps
    }

    fn is_def_equal<'x>(&self, env: &mut Environment<'x>, other: &dyn Construct) -> TripleBool {
        if let Some(other) = downcast_construct::<Self>(other) {
            if self.dependencies.len() != other.dependencies.len() {
                return TripleBool::Unknown;
            }
            for (&left, &right) in self.dependencies.iter().zip(other.dependencies.iter()) {
                if env.is_def_equal(left, right) != TripleBool::True {
                    return TripleBool::Unknown;
                }
            }
            env.is_def_equal(self.base, other.base)
        } else {
            TripleBool::Unknown
        }
    }

    fn reduce<'x>(&self, _env: &mut Environment<'x>) -> ConstructDefinition<'x> {
        self.base.into()
    }

    fn substitute<'x>(
        &self,
        env: &mut Environment<'x>,
        substitutions: &Substitutions,
    ) -> ConstructDefinition<'x> {
        let base = env.substitute(self.base, substitutions);
        let dependencies = self
            .dependencies
            .iter()
            .copied()
            .map(|x| env.substitute(x, substitutions))
            .collect_vec();
        ConstructDefinition::Resolved(Self::new(base, dependencies).dyn_clone())
    }
}
