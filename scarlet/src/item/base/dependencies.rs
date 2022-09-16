mod context;
mod dependencies_struct;
mod dependency;
mod feature;
mod requirement;
mod tests;

pub use self::{context::*, dependencies_struct::*, dependency::*, feature::*, requirement::*};
use crate::{environment::Environment, item::resolvable::UnresolvedItemError};

pub type DepResult = Dependencies;

pub struct DependencyError {
    pub partial_deps: Dependencies,
    pub cause: UnresolvedItemError,
}

impl DependencyError {
    pub fn from_unresolved(original_error: UnresolvedItemError) -> Self {
        Self {
            partial_deps: Dependencies::new(),
            cause: original_error,
        }
    }
}

impl Environment {}
