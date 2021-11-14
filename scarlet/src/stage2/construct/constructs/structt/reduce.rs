use super::CStruct;
use crate::stage2::structure::{ConstructId, Environment};

pub fn implementation<'x>(
    this: &CStruct<'x>,
    this_id: ConstructId<'x>,
    env: &mut Environment<'x>,
) -> ConstructId<'x> {
    todo!()
}
