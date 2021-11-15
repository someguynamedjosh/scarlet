mod dependencies;
mod reduce;
mod substitute;

use super::Substitutions;
use crate::stage2::{
    construct::Construct,
    dependencies::DepQueryResult,
    structure::{ConstructId, Environment},
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct StructField<'x> {
    pub name: Option<&'x str>,
    pub value: ConstructId<'x>,
}

#[derive(Debug)]
pub struct CStruct<'x>(pub Vec<StructField<'x>>);

impl<'x> Construct<'x> for CStruct<'x> {
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
