use super::{ConstructId, Environment};
use crate::constructs::substitution::Substitutions;

impl<'x> Environment<'x> {
    pub fn substitute(
        &mut self,
        con_id: ConstructId,
        substitutions: &Substitutions,
    ) -> ConstructId {
        let result = self
            .get_construct_definition(con_id)
            .dyn_clone()
            .substitute(self, substitutions);
        if result != con_id {
            let original_scope = self.get_construct_scope(con_id);
            let original_parent = self.get_scope(original_scope).parent;
            let result_scope = self.get_construct_scope(result);
        }
        result
    }
}
