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
            let original_scope = self.get_construct(con_id).scope.dyn_clone();
            self.set_scope(result, &*original_scope);
        }
        result
    }
}
