use super::CBuiltinValue;
use crate::stage2::{
    construct::constructs::Substitutions,
    structure::{ConstructId, Environment},
};

pub fn implementation<'x>(
    this: &CBuiltinValue,
    substitutions: &Substitutions<'x>,
    env: &mut Environment<'x>,
) -> ConstructId<'x> {
    todo!()
}
