use std::collections::HashMap;

use self::helpers::Context;
use crate::{
    shared::ItemId,
    stage2::{self},
    stage3::structure::Environment,
};

mod convert;
mod helpers;

pub fn ingest(src: stage2::structure::Environment) -> Result<Environment, String> {
    let mut ctx = Context {
        src,
        stage2_to_stage3: HashMap::new(),
        stage3_items: Vec::new(),
        info_requests: Vec::new(),
        next_stage3_id: ItemId(0),
    };
    let mut id = ItemId(0);
    while id.0 < ctx.src.items.len() {
        ctx.convert_iid(id)?;
        id.0 += 1;
    }

    let mut env = Environment::new();
    ctx.stage3_items.sort_unstable_by_key(|k| k.0);
    let items = ctx.stage3_items;
    let mut next_expected_id = ItemId(0);
    for (id, def) in items {
        assert_eq!(id, next_expected_id);
        env.insert(def);
        next_expected_id.0 += 1;
    }
    for (item, scope) in ctx.info_requests {
        env.get_mut(item).info_requested_in.push(scope);
    }

    Ok(env)
}
