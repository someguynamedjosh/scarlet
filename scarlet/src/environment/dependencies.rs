use super::{ConstructId, Environment};
use crate::constructs::variable::CVariable;

#[derive(Debug)]
pub struct Dependencies {
    eager: Vec<CVariable>,
}

impl Dependencies {
    pub fn new() -> Self {
        Self { eager: Vec::new() }
    }

    pub fn push_eager(&mut self, dep: CVariable) {
        for var in &self.eager {
            if dep.is_same_variable_as(var) {
                return;
            }
        }
        self.eager.push(dep);
    }

    pub fn as_variables(&self) -> impl Iterator<Item = &CVariable> {
        self.eager.iter()
    }

    pub fn into_variables(self) -> impl Iterator<Item = CVariable> {
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

    pub fn remove(&mut self, var: &CVariable) {
        self.eager = std::mem::take(&mut self.eager)
            .into_iter()
            .filter(|x| x != var)
            .collect();
    }

    pub fn pop_front(&mut self) -> CVariable {
        self.eager.remove(0)
    }
}

impl<'x> Environment<'x> {
    pub fn get_dependencies(&mut self, con_id: ConstructId) -> Dependencies {
        self.get_original_construct_definition(con_id)
            .dyn_clone()
            .get_dependencies(self)
    }

    pub fn get_non_capturing_dependencies(&mut self, _con_id: ConstructId) -> Vec<CVariable> {
        todo!()
    }
}
