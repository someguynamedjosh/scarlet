use std::collections::{BTreeSet, HashSet};

use maplit::hashset;

use super::Dependency;
use crate::item::{definitions::variable::VariableId, resolvable::UnresolvedItemError, ItemPtr};

#[derive(Clone, Debug, Default)]
pub struct Dependencies {
    dependencies: BTreeSet<Dependency>,
    skipped_due_to_recursion: HashSet<ItemPtr>,
    skipped_due_to_unresolved: Option<UnresolvedItemError>,
}

impl Dependencies {
    pub fn new() -> Self {
        Self {
            dependencies: BTreeSet::new(),
            skipped_due_to_recursion: HashSet::new(),
            skipped_due_to_unresolved: None,
        }
    }

    pub fn new_missing(item: ItemPtr) -> Self {
        Self {
            dependencies: BTreeSet::new(),
            skipped_due_to_recursion: hashset![item],
            skipped_due_to_unresolved: None,
        }
    }

    pub fn new_error(error: UnresolvedItemError) -> Self {
        Self {
            dependencies: BTreeSet::new(),
            skipped_due_to_recursion: HashSet::new(),
            skipped_due_to_unresolved: Some(error),
        }
    }

    pub fn push_eager(&mut self, dep: Dependency) {
        if self.skipped_due_to_unresolved.is_some() {
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
        if self.skipped_due_to_unresolved.is_some() {
            return;
        }
        for &new_missing in other.missing() {
            self.skipped_due_to_recursion.insert(new_missing);
        }
        self.skipped_due_to_unresolved = other.skipped_due_to_unresolved;
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
        &self.skipped_due_to_recursion
    }

    pub fn error(&self) -> Option<UnresolvedItemError> {
        self.skipped_due_to_unresolved
    }
}
