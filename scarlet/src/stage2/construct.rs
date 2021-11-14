pub mod constructs;

use std::fmt::Debug;

use super::{
    dependencies::DepQueryResult,
    matchh::MatchResult,
    structure::{ConstructId, Environment, VariableId},
};
use crate::shared::OrderedMap;

pub enum BasicVarType<'x> {
    _32U,
    Bool,
    Just(ConstructId<'x>),
}

pub enum FullVarType<'x> {
    Basic(BasicVarType<'x>),
    And(Box<FullVarType<'x>>, Box<FullVarType<'x>>),
    Or(Box<FullVarType<'x>>, Box<FullVarType<'x>>),
}

impl<'x> From<BasicVarType<'x>> for FullVarType<'x> {
    fn from(input: BasicVarType<'x>) -> Self {
        Self::Basic(input)
    }
}

pub type Substitutions<'x> = OrderedMap<VariableId<'x>, ConstructId<'x>>;

pub fn vt_just<'x>(item: ConstructId<'x>) -> FullVarType<'x> {
    FullVarType::Basic(BasicVarType::Just(item))
}

pub trait Construct<'x>: Debug {
    fn dependencies(
        &self,
        env: &mut Environment<'x>,
        num_struct_unwraps: u32,
    ) -> DepQueryResult<'x>;

    fn reduce(&self, self_id: ConstructId<'x>, env: &mut Environment<'x>) -> ConstructId<'x>;

    fn substitute(
        &self,
        substitutions: &Substitutions<'x>,
        env: &mut Environment<'x>,
    ) -> ConstructId<'x>;

    fn vomit(&self, env: &Environment<'x>) -> ! {
        todo!()
    }
}
