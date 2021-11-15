mod others;
mod variable;

use super::structures::DepQueryResult;
use crate::stage2::{
    construct::Construct,
    structure::{ConstructId, Environment},
};

impl<'x> Environment<'x> {
    pub(super) fn get_deps_from_def(
        &mut self,
        of: ConstructId<'x>,
        num_struct_unwraps: u32,
    ) -> DepQueryResult<'x> {
        self.get_definition(of)
            .dependencies(self, num_struct_unwraps)
    }
}
