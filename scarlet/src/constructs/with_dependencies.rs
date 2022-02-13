use itertools::Itertools;

use super::{
    base::{Construct, ConstructId},
    GenInvResult,
};
use crate::{
    environment::{
        def_equal::DefEqualResult,
        dependencies::{DepResult, Dependencies},
        sub_expr::{NestedSubstitutions, SubExpr},
        Environment,
    },
    impl_any_eq_for_construct,
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
    ) -> GenInvResult {
        env.generated_invariants(self.base)
    }

    fn get_dependencies<'x>(&self, env: &mut Environment<'x>) -> DepResult {
        let mut deps = Dependencies::new();
        let base_deps = env.get_dependencies(self.base);
        for &priority_dep in &self.dependencies {
            let priority_dep_deps = env.get_dependencies(priority_dep);
            let priority_dep_error = priority_dep_deps.error();
            for dep in priority_dep_deps.into_variables() {
                if base_deps.contains(&dep) {
                    deps.push_eager(dep);
                } else if let Some(err) = priority_dep_error {
                    deps.append(Dependencies::new_error(err));
                } else if let Some(err) = base_deps.error() {
                    // If the base had an error, we might not be accounting for
                    // all the dependencies it has. We might be throwing out a
                    // priority dep that it actually has, so we need to
                    // terminate the dependency list now before anything else
                    // gets added out of order.
                    deps.append(Dependencies::new_error(err));
                }
            }
            if let Some(err) = priority_dep_error {
                deps.append(Dependencies::new_error(err));
            }
        }
        deps.append(base_deps);
        deps
    }

    fn asymm_is_def_equal<'x>(
        &self,
        env: &mut Environment<'x>,
        subs: &NestedSubstitutions,
        other: SubExpr,
        recursion_limit: u32,
    ) -> DefEqualResult {
        env.is_def_equal(SubExpr(self.base, subs), other, recursion_limit)
    }
}
