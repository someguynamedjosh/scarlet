use self::expression::ingest_expression;
use crate::{
    shared::{Definitions, ItemId},
    stage1::structure::expression::Expression,
    stage2::{ingest::context::Context, structure::Environment},
};

mod context;
mod definitions;
mod expression;
mod postfix_construct;
mod replacements;
mod root_construct;

pub fn ingest(
    env: &mut Environment,
    expression: Expression,
    into: ItemId,
    scopes: &[&Definitions],
    extra_defines: Definitions,
) -> Result<ItemId, String> {
    let mut ctx = Context::new(env).with_current_item_id(into);
    for scope in scopes {
        ctx = ctx.with_additional_scope(scope);
    }
    let root_scope = ingest_expression(&mut ctx, expression, extra_defines)?;

    Ok(root_scope)
}
