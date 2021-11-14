use super::COperation;
use crate::stage2::{dependencies::DepQueryResult, structure::Environment};

pub fn implementation<'x>(
    this: &COperation<'x>,
    env: &mut Environment<'x>,
    num_struct_unwraps: u32,
) -> DepQueryResult<'x> {
    let mut result = DepQueryResult::new();
    for arg in &this.args {
        result.append(env.dep_query(*arg, num_struct_unwraps));
    }
    result
}
