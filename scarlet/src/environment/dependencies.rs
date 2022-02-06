use super::{ConstructId, Environment};
use crate::constructs::variable::{CVariable, Dependency, VariableId};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DepResStackFrame(pub(super) ConstructId);
pub type DepResStack = Vec<DepResStackFrame>;

#[derive(Debug)]
pub struct Dependencies {
    eager: Vec<Dependency>,
}

impl Dependencies {
    pub fn new() -> Self {
        Self { eager: Vec::new() }
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

    pub(crate) fn contains(&self, dep: &Dependency) -> bool {
        for target in &self.eager {
            if target == dep {
                return true;
            }
        }
        false
    }

    pub(crate) fn contains_var(&self, dep: VariableId) -> bool {
        for target in &self.eager {
            if target.id == dep {
                return true;
            }
        }
        false
    }
}

impl<'x> Environment<'x> {
    pub fn get_dependencies(&mut self, con_id: ConstructId) -> Dependencies {
        if self.dep_res_stack.iter().any(|i| i.0 == con_id) {
            Dependencies::new()
        } else {
            let con = self.get_construct_definition(con_id).dyn_clone();
            self.dep_res_stack.push(DepResStackFrame(con_id));
            let deps = con.get_dependencies(self);
            assert_eq!(self.dep_res_stack.pop(), Some(DepResStackFrame(con_id)));
            deps
        }
    }
}
