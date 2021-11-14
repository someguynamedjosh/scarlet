use super::CUnresolved;
use crate::stage2::{dependencies::DepQueryResult, structure::Environment};

pub fn implementation<'x>(
    this: &CUnresolved<'x>,
    env: &mut Environment<'x>,
    num_struct_unwraps: u32,
) -> DepQueryResult<'x> {
    todo!()
}
