use super::{Dcc, DepResult, Dependencies, OnlyCalledByDcc};
use crate::item::ItemPtr;

/// A required component of ItemDefinition.
pub trait DependenciesFeature {
    #[allow(unused_variables)]
    fn get_dependencies_using_context(
        &self,
        this: &ItemPtr,
        ctx: &mut Dcc,
        affects_return_value: bool,
        _: OnlyCalledByDcc,
    ) -> DepResult {
        Dependencies::new()
    }
}
