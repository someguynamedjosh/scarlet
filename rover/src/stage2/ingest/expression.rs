use super::{
    context::Context,
    postfix_construct::{ingest_defining_construct, ingest_postfix_construct},
    root_construct::ingest_root_construct,
};
use crate::{
    shared::{Definitions, ItemId},
    stage1::structure::{construct::Construct, expression::Expression},
    stage2::structure::UnresolvedItem,
};

fn convert_expression_to_item(
    ctx: &mut Context,
    mut expr: Expression,
    extra_defines: Definitions,
) -> Result<UnresolvedItem, String> {
    if expr.others.iter().all(|c| c.label != "defining") && extra_defines.len() > 0 {
        let fake_post = Construct::parser(false)("defining{}").unwrap().1;
        ingest_defining_construct(ctx, fake_post, expr, extra_defines)
    } else if let Some(post) = expr.others.pop() {
        ingest_postfix_construct(ctx, post, expr, extra_defines)
    } else {
        let root = expr.root;
        ingest_root_construct(ctx, root)
    }
}

fn define_or_dereference_item(ctx: &mut Context, item: UnresolvedItem) -> ItemId {
    if let Some(id) = ctx.current_item_id {
        ctx.environment.define(id, item);
        id
    } else if let UnresolvedItem::Item(id) = item {
        id
    } else {
        ctx.environment.insert_unresolved_item(item)
    }
}

pub(super) fn ingest_expression(
    ctx: &mut Context,
    expr: Expression,
    extra_defines: Definitions,
) -> Result<ItemId, String> {
    let item = convert_expression_to_item(ctx, expr, extra_defines)?;
    Ok(define_or_dereference_item(ctx, item))
}
