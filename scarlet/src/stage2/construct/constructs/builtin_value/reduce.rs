use super::CBuiltinValue;
use crate::stage2::structure::{ConstructId, Environment};

pub fn implementation<'x>(
    this: &CBuiltinValue<'x>,
    this_id: ConstructId<'x>,
    env: &mut Environment<'x>,
) -> ConstructId<'x> {
    todo!()
}
