mod dependencies;
mod reduce;
mod substitute;

use crate::{
    shared::OrderedMap,
    stage2::{
        construct::Construct,
        dependencies::DepQueryResult,
        structure::{ConstructId, Environment, VariableId},
    },
};

pub type Substitutions<'x> = OrderedMap<VariableId<'x>, ConstructId<'x>>;

#[derive(Debug)]
pub struct CSubstitute<'x>(pub ConstructId<'x>, pub Substitutions<'x>);

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
