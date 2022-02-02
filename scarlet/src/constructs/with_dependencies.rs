use itertools::Itertools;

use super::{
    base::{Construct, ConstructId},
    downcast_construct,
    substitution::{Substitutions, NestedSubstitutions, SubExpr},
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

    fn is_def_equal<'x>(
        &self,
        env: &mut Environment<'x>,
        subs: &NestedSubstitutions,
        other: SubExpr,
    ) -> TripleBool {
        env.is_def_equal(SubExpr(self.base, subs), other)
    }
}
