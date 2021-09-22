use super::{
    context::Context, postfix_construct::ingest_postfix_construct,
    root_construct::ingest_root_construct,
};
use crate::{shared::ItemId, stage1::structure::expression::Expression, stage2::structure::UnresolvedItem};

fn convert_expression_to_item(ctx: &mut Context, mut expr: Expression) -> Result<UnresolvedItem, String> {
    if let Some(post) = expr.others.pop() {
        ingest_postfix_construct(ctx, post, expr)
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
        ctx.environment.insert(item)
    }
}

pub(super) fn ingest_expression(ctx: &mut Context, expr: Expression) -> Result<ItemId, String> {
    let item = convert_expression_to_item(ctx, expr)?;
    Ok(define_or_dereference_item(ctx, item))
}
