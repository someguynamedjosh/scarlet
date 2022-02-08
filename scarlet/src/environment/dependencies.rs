use std::collections::HashSet;

use maplit::hashset;

use super::{ConstructId, Environment, UnresolvedConstructError};
use crate::constructs::variable::{CVariable, Dependency, VariableId};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DepResStackFrame(pub(super) ConstructId);
pub type DepResStack = Vec<DepResStackFrame>;

#[derive(Debug, Default)]
pub struct Dependencies {
    eager: Vec<Dependency>,
    /// Signifies this dependency list was built without considering the full
    /// list of dependencies for each contained construct, due to that item
    /// recursively depending on itself.
    missing: HashSet<ConstructId>,
}

impl Dependencies {
    pub fn new() -> Self {
        Self {
            eager: Vec::new(),
            missing: HashSet::new(),
        }
    }

    pub fn new_missing(con: ConstructId) -> Self {
        Self {
            eager: Vec::new(),
            missing: hashset![con],
        }
    }

    pub fn push_eager(&mut self, dep: Dependency) {
        for var in &self.eager {
            if &dep == var {
                return;
            }
        }
        self.eager.push(dep);
    }

    pub fn as_variables(&self) -> impl Iterator<Item = &Dependency> {
        self.eager.iter()
    }

    pub fn into_variables(self) -> impl Iterator<Item = Dependency> {
        self.eager.into_iter()
    }

    pub fn append(&mut self, other: Dependencies) {
        for &new_missing in other.missing() {
            self.missing.insert(new_missing);
        }
        for eager in other.into_variables() {
            self.push_eager(eager);
        }
    }

    pub fn num_variables(&self) -> usize {
        self.as_variables().count()
    }

    pub fn remove(&mut self, var: VariableId) {
        self.eager = std::mem::take(&mut self.eager)
            .into_iter()
            .filter(|x| x.id != var)
            .collect();
    }

    pub fn pop_front(&mut self) -> Dependency {
        self.eager.remove(0)
    }

    pub fn contains(&self, dep: &Dependency) -> bool {
        for target in &self.eager {
            if target == dep {
                return true;
            }
        }
        false
    }

    pub fn contains_var(&self, dep: VariableId) -> bool {
        for target in &self.eager {
            if target.id == dep {
                return true;
            }
        }
        false
    }

    pub fn missing(&self) -> &HashSet<ConstructId> {
        &self.missing
    }
}

pub struct DependencyError {
    pub partial_deps: Dependencies,
    pub cause: UnresolvedConstructError,
}

impl DependencyError {
    pub fn from_unresolved(original_error: UnresolvedConstructError) -> Self {
        Self {
            partial_deps: Dependencies::new(),
            cause: original_error,
        }
    }
}

pub type DepResult = Result<Dependencies, DependencyError>;

pub fn get_partial_or_full_deps(
    from: DepResult,
) -> (Dependencies, Option<UnresolvedConstructError>) {
    match from {
        Ok(deps) => (deps, None),
        Err(error) => (error.partial_deps, Some(error.cause)),
    }
}

impl<'x> Environment<'x> {
    pub fn get_dependencies(&mut self, con_id: ConstructId) -> DepResult {
        if self.dep_res_stack.iter().any(|i| i.0 == con_id) {
            Ok(Dependencies::new_missing(con_id))
        } else {
            let con = match self.get_construct_definition(con_id) {
                Ok(ok) => ok.dyn_clone(),
                Err(err) => return Err(DependencyError::from_unresolved(err)),
            };
            self.dep_res_stack.push(DepResStackFrame(con_id));
            let deps = con.get_dependencies(self);
            assert_eq!(self.dep_res_stack.pop(), Some(DepResStackFrame(con_id)));
            deps.map(|mut ok| {
                ok.missing.remove(&con_id);
                ok
            })
        }
    }
}
