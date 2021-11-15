mod dependencies;
mod reduce;
mod substitute;

use std::any::Any;

use super::Substitutions;
use crate::stage2::{
    construct::Construct,
    dependencies::DepQueryResult,
    structure::{ConstructId, Environment},
};

#[derive(Debug, PartialEq)]
pub struct CTemplate<'x> {
    __: &'x (),
}

impl<'x> Construct<'x> for CTemplate<'x> {
    fn dependencies(
        &self,
        env: &mut Environment<'x>,
        num_struct_unwraps: u32,
    ) -> DepQueryResult<'x> {
        dependencies::implementation(self, env, num_struct_unwraps)
    }

    fn eq(&self, other: &(impl Construct<'x> + 'x)) -> bool {
        (other as &dyn Any)
            .downcast_ref()
            .map(|other| self == other)
            .unwrap_or(false)
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
