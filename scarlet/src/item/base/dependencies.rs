mod context;
mod dependencies_struct;
mod feature;

use std::collections::{BTreeSet, HashSet};

use maplit::hashset;

pub use self::{context::*, dependencies_struct::*, feature::*};
use super::ItemPtr;
use crate::{
    environment::{Environment, UnresolvedItemError},
    item::{
        variable::{Dependency, VariableId},
        ItemDefinition,
    },
};

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
