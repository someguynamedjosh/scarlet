pub mod constructs;

use std::{any::Any, fmt::Debug};

use self::constructs::Substitutions;
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

pub fn vt_just<'x>(item: ConstructId<'x>) -> FullVarType<'x> {
    FullVarType::Basic(BasicVarType::Just(item))
}

pub type BoxedConstruct<'x> = Box<dyn Construct<'x> + 'x>;

pub trait Construct<'x>: Debug {
    fn dependencies(
        &self,
        env: &mut Environment<'x>,
        num_struct_unwraps: u32,
    ) -> DepQueryResult<'x>;

    fn eq(&self, other: &(dyn Construct<'x> + 'x)) -> bool;

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
