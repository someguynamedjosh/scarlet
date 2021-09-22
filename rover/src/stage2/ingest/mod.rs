use super::structure::Definitions;
use crate::{
    stage1::structure::statement::Statement,
    stage2::{
        ingest::{
            context::Context, rover_item::define_rover_item, statements::process_definitions,
        },
        structure::{Environment, Item, ItemId},
    },
};

mod context;
mod expression;
mod helpers;
mod postfix_construct;
mod root_construct;
mod rover_item;
mod statements;

fn define_root_scope(env: &mut Environment, god_type: ItemId, definitions: Definitions) -> ItemId {
    let root_scope = env.next_id();
    env.mark_as_module(root_scope);
    env.define(
        root_scope,
        Item::Defining {
            base: god_type,
            definitions,
        },
    );
    root_scope
}

pub fn ingest(statements: Vec<Statement>) -> Result<(Environment, ItemId), String> {
    let mut env = Environment::new();
    let (rover, god_type) = define_rover_item(&mut env);
    let rover_def = (format!("rover"), rover);

    let mut ctx = Context::new(&mut env);
    let definitions = process_definitions(&mut ctx, statements, vec![rover_def])?;
    let root_scope = define_root_scope(&mut env, god_type, definitions);

    Ok((env, root_scope))
}
