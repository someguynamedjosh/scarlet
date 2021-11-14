use super::COperation;
use crate::stage2::{
    construct::Substitutions,
    structure::{ConstructId, Environment},
};

pub fn implementation<'x>(
    this: &COperation<'x>,
    substitutions: &Substitutions<'x>,
    env: &mut Environment<'x>,
) -> ConstructId<'x> {
    todo!()
}
