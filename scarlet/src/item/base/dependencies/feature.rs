use super::{Dcc, DepResult, OnlyCalledByDcc};

/// A required component of ItemDefinition.
pub trait DependenciesFeature {
    fn get_dependencies_using_context(&self, ctx: &mut Dcc, _: OnlyCalledByDcc) -> DepResult;
}
