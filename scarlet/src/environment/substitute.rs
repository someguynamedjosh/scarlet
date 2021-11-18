use super::{ConstructDefinition, ConstructId, Environment};
use crate::{
    constructs::{self, builtin_value::CBuiltinValue, substitution::Substitutions},
    tokens::structure::Token,
};

impl<'x> Environment<'x> {
    pub fn substitute(
        &mut self,
        con_id: ConstructId,
        substitutions: &Substitutions,
    ) -> ConstructId {
        self.get_construct(con_id)
            .dyn_clone()
            .substitute(self, substitutions)
    }
}
