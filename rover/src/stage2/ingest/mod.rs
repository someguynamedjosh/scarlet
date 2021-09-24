use crate::{
    shared::{Definitions, Item, ItemId},
    stage1::structure::statement::Statement,
    stage2::{
        ingest::{
            context::Context, definitions::process_definitions, rover_item::define_rover_item,
        },
        structure::Environment,
    },
};

mod context;
mod definitions;
mod expression;
mod helpers;
mod postfix_construct;
mod replacements;
mod root_construct;
mod rover_item;

fn define_root_scope(
    env: &mut Environment,
    root_scope: ItemId,
    god_type: ItemId,
    definitions: Definitions,
) -> ItemId {
    env.mark_as_scope(root_scope);
    env.define(
        root_scope,
        Item::Defining {
            base: god_type,
            definitions,
        }
        .into(),
    );
    root_scope
}

pub fn ingest(statements: Vec<Statement>) -> Result<(Environment, ItemId), String> {
    let mut env = Environment::new();
    let root_scope = env.next_id();
    let (rover, god_type) = define_rover_item(&mut env);
    let rover_def = ("rover".to_string(), rover);

    let mut ctx = Context::new(&mut env);
    let definitions = process_definitions(&mut ctx, statements, vec![rover_def], root_scope)?;
    define_root_scope(&mut env, root_scope, god_type, definitions);
    env.set_defined_in(rover, root_scope);

    Ok((env, root_scope))
}
