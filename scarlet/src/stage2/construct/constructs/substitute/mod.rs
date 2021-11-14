mod dependencies;
mod reduce;
mod substitute;

use crate::stage2::{
    construct::{Construct, Substitutions},
    dependencies::DepQueryResult,
    structure::{ConstructId, Environment},
};

pub struct CSubstitute<'x> {
    __: &'x (),
}

impl<'x> Construct<'x> for CSubstitute<'x> {
    fn dependencies(
        &self,
        env: &mut Environment<'x>,
        num_struct_unwraps: u32,
    ) -> DepQueryResult<'x> {
        dependencies::implementation(self, env, num_struct_unwraps)
    }

    fn reduce(&self, self_id: ConstructId<'x>, env: &mut Environment<'x>) -> ConstructId<'x> {
        reduce::implementation(self, self_id, env)
    }

    fn substitute(
        &self,
        substitutions: &Substitutions<'x>,
        env: &mut Environment<'x>,
    ) -> ConstructId<'x> {
        substitute::implementation(self, substitutions, env)
    }
}