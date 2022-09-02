use std::collections::{BTreeSet, HashSet};

use maplit::hashset;

use super::{requirement::Requirement, Dependency};
use crate::{
    item::{definitions::variable::VariablePtr, resolvable::UnresolvedItemError, ItemPtr},
    util::PtrExtension,
};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Dependencies {
    pub(super) dependencies: BTreeSet<Dependency>,
    pub(super) requirements: BTreeSet<Requirement>,
    pub(super) skipped_due_to_recursion: HashSet<ItemPtr>,
    pub(super) skipped_due_to_unresolved: Option<UnresolvedItemError>,
}

impl Dependencies {
    pub fn new() -> Self {
        Self {
            dependencies: BTreeSet::new(),
            requirements: BTreeSet::new(),
            skipped_due_to_recursion: HashSet::new(),
            skipped_due_to_unresolved: None,
        }
    }

    pub fn new_missing(item: ItemPtr) -> Self {
        Self {
            dependencies: BTreeSet::new(),
            requirements: BTreeSet::new(),
            skipped_due_to_recursion: hashset![item],
            skipped_due_to_unresolved: None,
        }
    }

    pub fn new_error(error: UnresolvedItemError) -> Self {
        Self {
            dependencies: BTreeSet::new(),
            requirements: BTreeSet::new(),
            skipped_due_to_recursion: HashSet::new(),
            skipped_due_to_unresolved: Some(error),
        }
    }

    pub fn push_value(&mut self, dep: Dependency) {
        if self.skipped_due_to_unresolved.is_some() {
            return;
        }
        for other_dep in &self.dependencies {
            if dep.var.is_same_instance_as(&other_dep.var)
                && other_dep.affects_return_value >= dep.affects_return_value
            {
                return;
            }
        }
        self.dependencies.replace(dep);
    }

    pub fn push_requirement(&mut self, req: Requirement) {
        if self.skipped_due_to_unresolved.is_some() {
            return;
        }
        self.requirements.replace(req);
    }

    pub fn as_variables(&self) -> impl Iterator<Item = &Dependency> {
        self.dependencies.iter()
    }

    pub fn as_requirements(&self) -> impl Iterator<Item = &Requirement> {
        self.requirements.iter()
    }

    #[track_caller]
    pub fn as_complete_variables(
        &self,
    ) -> Result<
        (
            impl Iterator<Item = &Dependency>,
            impl Iterator<Item = &Requirement>,
        ),
        UnresolvedItemError,
    > {
        if let Some(err) = self.error() {
            Err(err.clone())
        } else {
            Ok((self.dependencies.iter(), self.requirements.iter()))
        }
    }

    pub fn into_variables(self) -> impl Iterator<Item = Dependency> {
        self.dependencies.into_iter()
    }

    pub fn append(&mut self, other: Dependencies) {
        if self.skipped_due_to_unresolved.is_some() {
            return;
        }
        for new_missing in other.missing() {
            self.skipped_due_to_recursion
                .insert(new_missing.ptr_clone());
        }
        self.skipped_due_to_unresolved = other.skipped_due_to_unresolved.clone();
        for dep in other.as_variables() {
            self.push_value(dep.clone());
        }
        for req in other.as_requirements() {
            self.push_requirement(req.clone());
        }
    }

    pub fn num_variables(&self) -> usize {
        self.as_variables().count()
    }

    pub fn remove(&mut self, var: &VariablePtr) {
        self.dependencies = std::mem::take(&mut self.dependencies)
            .into_iter()
            .filter(|x| !x.var.is_same_instance_as(var))
            .collect();
        self.requirements = std::mem::take(&mut self.requirements)
            .into_iter()
            .filter(|x| !x.var.is_same_instance_as(var))
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

    pub fn contains_var(&self, dep: &VariablePtr) -> bool {
        for target in &self.dependencies {
            if target.var.is_same_instance_as(dep) {
                return true;
            }
        }
        false
    }

    pub fn get_var(&self, dep: &VariablePtr) -> Option<&Dependency> {
        for target in &self.dependencies {
            if target.var.is_same_instance_as(dep) {
                return Some(target);
            }
        }
        None
    }

    pub fn missing(&self) -> &HashSet<ItemPtr> {
        &self.skipped_due_to_recursion
    }

    pub fn error(&self) -> &Option<UnresolvedItemError> {
        &self.skipped_due_to_unresolved
    }
}
