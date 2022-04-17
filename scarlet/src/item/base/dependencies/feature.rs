use super::{Dcc, DepResult, Dependencies, OnlyCalledByDcc};

/// A required component of ItemDefinition.
pub trait DependenciesFeature {
    #[allow(unused_variables)]
    fn get_dependencies_using_context(&self, ctx: &mut Dcc, _: OnlyCalledByDcc) -> DepResult {
        Dependencies::new()
    }
}
