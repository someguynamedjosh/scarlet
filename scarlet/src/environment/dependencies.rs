use super::{ConstructDefinition, ConstructId, Environment};
use crate::{
    constructs::{
        self, builtin_value::CBuiltinValue, substitution::Substitutions, variable::CVariable,
    },
    tokens::structure::Token,
};

impl<'x> Environment<'x> {
    pub fn get_dependencies(&mut self, con_id: ConstructId) -> Vec<CVariable> {
        self.get_construct(con_id)
            .dyn_clone()
            .get_dependencies(self)
    }

    pub fn get_non_capturing_dependencies(&mut self, con_id: ConstructId) -> Vec<CVariable> {
        self.get_dependencies(con_id)
            .into_iter()
            .filter(|x| !x.capturing)
            .collect()
    }
}
