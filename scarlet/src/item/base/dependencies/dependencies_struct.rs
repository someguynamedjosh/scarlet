use std::collections::{BTreeSet, HashSet};

use maplit::hashset;

use super::ItemPtr;
use crate::{
    environment::{Environment, UnresolvedItemError},
    item::{
        variable::{Dependency, VariableId},
        ItemDefinition,
    },
};

#[derive(Clone, Debug, Default)]
pub struct Dependencies {
    dependencies: BTreeSet<Dependency>,
    /// Signifies this dependency list was built without considering the full
    /// list of dependencies for each contained construct, due to that item
    /// recursively depending on itself.
    missing: HashSet<ItemPtr>,
    /// Signifies this dependency list is missing all the dependencies from a
    /// particular item and any dependencies after it.
    error: Option<UnresolvedItemError>,
}

impl Dependencies {
    pub fn new() -> Self {
        Self {
            dependencies: BTreeSet::new(),
            missing: HashSet::new(),
            error: None,
        }
    }

    pub fn new_missing(item: ItemPtr) -> Self {
        Self {
            dependencies: BTreeSet::new(),
            missing: hashset![item],
            error: None,
        }
    }

    pub fn new_error(error: UnresolvedItemError) -> Self {
        Self {
            dependencies: BTreeSet::new(),
            missing: HashSet::new(),
            error: Some(error),
        }
    }

    pub fn push_eager(&mut self, dep: Dependency) {
        if self.error.is_some() {
            return;
        }
        for var in &self.dependencies {
            if &dep == var {
                return;
            }
        }
        self.dependencies.insert(dep);
    }

    pub fn as_variables(&self) -> impl Iterator<Item = &Dependency> {
        self.dependencies.iter()
    }

    pub fn into_variables(self) -> impl Iterator<Item = Dependency> {
        self.dependencies.into_iter()
    }

    pub fn append(&mut self, other: Dependencies) {
        if self.error.is_some() {
            return;
        }
        for &new_missing in other.missing() {
            self.missing.insert(new_missing);
        }
        self.error = other.error;
        for eager in other.into_variables() {
            self.push_eager(eager);
        }
    }

    pub fn num_variables(&self) -> usize {
        self.as_variables().count()
    }

    pub fn remove(&mut self, var: VariableId) {
        self.dependencies = std::mem::take(&mut self.dependencies)
            .into_iter()
            .filter(|x| x.id != var)
            .collect();
    }

    pub fn pop_front(&mut self) -> Dependency {
        self.dependencies.pop_first().unwrap()
    }

    pub fn contains(&self, dep: &Dependency) -> bool {
        for target in &self.dependencies {
            if target == dep {
                return true;
            }
        }
        false
    }

    pub fn contains_var(&self, dep: VariableId) -> bool {
        for target in &self.dependencies {
            if target.id == dep {
                return true;
            }
        }
        false
    }

    pub fn get_var(&self, dep: VariableId) -> Option<&Dependency> {
        for target in &self.dependencies {
            if target.id == dep {
                return Some(target);
            }
        }
        None
    }

    pub fn missing(&self) -> &HashSet<ItemPtr> {
        &self.missing
    }

    pub fn error(&self) -> Option<UnresolvedItemError> {
        self.error
    }
}
