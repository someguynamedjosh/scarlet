use std::collections::HashMap;

use self::helpers::{convert_defined_in, convert_iid, full_convert_iid};
use crate::{
    shared::{Item, ItemId},
    stage2::structure::{self as stage2, UnresolvedItem},
    stage3::{
        ingest::context::{convert_unresolved_item, convertible, Context},
        structure::Environment,
    },
};

mod context;
mod dereference;
mod dereferenced_item;
mod get_member;
mod helpers;
mod shared_items;

pub fn ingest(src: &stage2::Environment) -> Result<Environment, String> {
    let setup = IngestSetup::new().ingest(src);

    let mut env = Environment::new();
    let unconverted_items = setup.unconverted_items();
    let ctx = setup.into_context(src, &mut env);
    convert_items(ctx, unconverted_items)?;

    Ok(env)
}

struct IngestSetup {
    next_stage3_id: ItemId,
    stage2_to_stage3: HashMap<ItemId, ItemId>,
}

impl IngestSetup {
    fn new() -> Self {
        Self {
            next_stage3_id: ItemId(0),
            stage2_to_stage3: HashMap::new(),
        }
    }

    fn ingest(mut self, src: &stage2::Environment) -> Self {
        for (id, item) in src.iter() {
            let item = item.definition.as_ref().unwrap();
            if convertible(item) {
                self.stage2_to_stage3.insert(id, self.next_stage3_id);
                self.next_stage3_id.0 += 1
            }
        }
        self
    }

    fn unconverted_items(&self) -> Vec<ItemId> {
        let mut res: Vec<_> = self.stage2_to_stage3.keys().copied().collect();
        res.sort();
        res
    }

    fn into_context<'e>(
        self,
        src: &'e stage2::Environment,
        env: &'e mut Environment,
    ) -> Context<'e> {
        Context {
            env,
            extra_items: vec![],
            id_map: self.stage2_to_stage3,
            next_unused_id: self.next_stage3_id,
            src,
        }
    }
}

fn convert_items(mut ctx: Context, unconverted_items: Vec<ItemId>) -> Result<(), String> {
    for id in unconverted_items {
        convert_item(&mut ctx, id)?;
    }
    add_extra_items_to_env(&mut ctx);
    convert_metadata(&mut ctx)?;
    Ok(())
}

fn find_definition_in_output(ctx: &mut Context, item: ItemId) -> Result<Option<ItemId>, String> {
    let defined_in = ctx.src.items[item.0].defined_in;
    convert_defined_in(ctx, defined_in)
}

fn convert_item(ctx: &mut Context, item: ItemId) -> Result<(), String> {
    let def = ctx.src.definition_of(item).definition.as_ref().unwrap();
    let converted = convert_unresolved_item(ctx, def)?;
    let defined_in = find_definition_in_output(ctx, item)?;
    ctx.env.insert_item(converted, defined_in);
    Ok(())
}

/// During the conversion process, ctx may accumulate extra items which then
/// need to be placed into the environment. This is required because it is not
/// always possible to define an item before its ID is needed.
fn add_extra_items_to_env(ctx: &mut Context) {
    let mut id = ItemId(ctx.env.items.len());
    let items = std::mem::replace(&mut ctx.extra_items, vec![]);
    for (item, defined_in) in items {
        ctx.env.insert_item(item, defined_in);
        id.0 += 1;
    }
}

fn convert_metadata(ctx: &mut Context) -> Result<(), String> {
    for (id, old_def) in ctx.src.iter() {
        let new_id = full_convert_iid(ctx, id)?;
        add_extra_items_to_env(ctx);
        if old_def.info_requested {
            let new_def_in = convert_defined_in(ctx, old_def.defined_in)?;
            ctx.env.mark_info(new_id, new_def_in);
        }
        if old_def.is_scope {
            ctx.env.mark_as_scope(new_id);
        }
    }
    Ok(())
}
