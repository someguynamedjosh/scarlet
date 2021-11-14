use super::CUnresolved;
use crate::stage2::structure::{ConstructId, Environment};

pub fn implementation<'x>(
    this: &CUnresolved<'x>,
    this_id: ConstructId<'x>,
    env: &mut Environment<'x>,
) -> ConstructId<'x> {
    todo!()
}
