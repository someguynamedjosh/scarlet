use super::{ConstructId, Environment};
use crate::constructs::substitution::Substitutions;

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
