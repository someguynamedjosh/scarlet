use super::{ConstructId, Environment};
use crate::{constructs::substitution::Substitutions, shared::TripleBool};

impl<'x> Environment<'x> {
    pub fn substitute(
        &mut self,
        con_id: ConstructId,
        substitutions: &Substitutions,
    ) -> ConstructId {
        let def = self.get_construct_definition(con_id).dyn_clone();
        let scope = self.get_construct(con_id).scope.dyn_clone();
        let result = def.substitute(self, substitutions, scope);
        if self.is_def_equal(result, con_id) == TripleBool::True {
            con_id
        } else {
            result
        }
    }
}
