use crate::stage2::structure::{BuiltinOperation, ConstructId, Environment, VarType};

impl<'x> Environment<'x> {
    pub(in crate::stage2) fn find_bounding_pattern(
        &mut self,
        pattern: ConstructId<'x>,
    ) -> ConstructId<'x> {
        todo!()
    }
}
