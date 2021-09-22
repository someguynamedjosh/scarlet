use crate::{
    stage1::structure::statement::Statement,
    stage2::{
        helpers::Context,
        ingest::{rover_item::define_rover_item, statements::process_definitions},
        structure::{Environment, Item, ItemId},
    },
};

mod expression;
mod rover_item;
mod statements;

pub fn ingest(statements: Vec<Statement>) -> Result<(Environment, ItemId), String> {
    let mut env = Environment::new();
    let (rover, god_type) = define_rover_item(&mut env);
    let rover_def = (format!("rover"), rover);
    let definitions =
        process_definitions(statements, vec![rover_def], &mut env, Context::Plain, &[])?;
    let file_id = env.next_id();
    env.mark_as_module(file_id);
    env.define(
        file_id,
        Item::Defining {
            base: god_type,
            definitions,
        },
    );
    Ok((env, file_id))
}
