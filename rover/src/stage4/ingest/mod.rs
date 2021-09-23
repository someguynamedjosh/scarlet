use var_list::VarList;

use crate::{
    shared::ItemId,
    stage3::structure::{self as stage3},
    stage4::structure::Environment,
};

mod after_replacing;
mod compute_type;
mod helpers;
mod type_of_basics;
mod type_of_pick;
mod type_of_replacing;
mod var_list;

pub fn ingest(from: stage3::Environment) -> Result<Environment, String> {
    let mut env = Environment::new(from);
    let mut next_item = ItemId(0);
    while next_item.0 < env.items.len() {
        env.compute_type(next_item)?;
        next_item.0 += 1;
    }
    Ok(env)
}
