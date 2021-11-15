mod dependencies;
mod reduce;
mod substitute;

use super::Substitutions;
use crate::stage2::{
    construct::{BasicVarType, Construct, FullVarType},
    dependencies::DepQueryResult,
    matchh::MatchResult,
    structure::{ConstructId, Environment},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Operation {
    Sum32U,
    Difference32U,
    Product32U,
    Quotient32U,
    Modulo32U,
    Power32U,

    LessThan32U,
    LessThanOrEqual32U,
    GreaterThan32U,
    GreaterThanOrEqual32U,
}

#[derive(Debug)]
pub struct COperation<'x> {
    op: Operation,
    args: Vec<ConstructId<'x>>,
}

impl<'x> Construct<'x> for COperation<'x> {
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
