use std::collections::HashMap;

use crate::{
    shared::ItemId,
    stage2::structure::{self as stage2},
    stage3::{
        ingest::context::{convert_unresolved_item, convertible, Context},
        structure::Environment,
    },
};

mod context;
mod dereference;
mod dereferenced_item;
mod helpers;
mod shared_items;

pub fn ingest(src: &stage2::Environment) -> Result<Environment, String> {
    let mut new_id = ItemId(0);
    let mut id_map = HashMap::new();
    let mut to_convert = Vec::new();
    for (id, item) in src.iter() {
        let item = item.as_ref().unwrap();
        if convertible(item) {
            id_map.insert(id, new_id);
            to_convert.push(id);
            new_id.0 += 1
        }
    }
    let mut env = Environment::new();
    let mut ctx = Context {
        id_map,
        src,
        env: &mut env,
        next_unused_id: new_id,
        extra_items: vec![],
    };
    for id in to_convert {
        let def = src.definition_of(id).as_ref().unwrap();
        let converted = convert_unresolved_item(&mut ctx, def)?;
        ctx.env.insert(converted);
    }
    for item in ctx.extra_items {
        env.insert(item);
    }
    Ok(env)
}
