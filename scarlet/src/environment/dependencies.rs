use std::collections::{BTreeSet, HashSet};

use maplit::hashset;

use super::{Environment, ItemId, UnresolvedItemError};
use crate::constructs::variable::{Dependency, VariableId};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DepResStackFrame(pub(super) ItemId);
pub type DepResStack = Vec<DepResStackFrame>;

#[derive(Clone, Debug, Default)]
pub struct Dependencies {
    dependencies: BTreeSet<Dependency>,
    /// Signifies this dependency list was built without considering the full
    /// list of dependencies for each contained construct, due to that item
    /// recursively depending on itself.
    missing: HashSet<ItemId>,
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

    pub fn new_missing(item: ItemId) -> Self {
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

    pub fn missing(&self) -> &HashSet<ItemId> {
        &self.missing
    }

    pub fn error(&self) -> Option<UnresolvedItemError> {
        self.error
    }
}

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

pub type DepResult = Dependencies;

impl<'x> Environment<'x> {
    pub fn get_dependencies(&mut self, item_id: ItemId) -> DepResult {
        if self.dep_res_stack.iter().any(|i| i.0 == item_id) {
            Dependencies::new_missing(item_id)
        } else {
            let con = match self.get_item_as_construct(item_id) {
                Ok(ok) => ok.dyn_clone(),
                Err(err) => return Dependencies::new_error(err),
            };
            self.dep_res_stack.push(DepResStackFrame(item_id));
            let mut deps = con.get_dependencies(self);
            assert_eq!(self.dep_res_stack.pop(), Some(DepResStackFrame(item_id)));
            deps.missing.remove(&item_id);
            deps
        }
    }
}
