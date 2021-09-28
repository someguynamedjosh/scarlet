use crate::{
    shared::{Item, ItemId},
    stage1::structure::construct::Construct,
    stage2::{
        ingest::{
            context::Context, expression::ingest_expression,
            postfix_construct::from::ingest_from_construct, replacements::ingest_replacements,
        },
        structure::UnresolvedItem,
    },
};

pub fn ingest_non_defining_postfix_construct(
    ctx: &mut Context,
    base_id: ItemId,
    post: Construct,
) -> Result<UnresolvedItem, String> {
    match &post.label[..] {
        "replacing" => ingest_replacing_construct(ctx, base_id, post),
        "member" => ingest_member_construct(base_id, post),
        "From" => ingest_from_construct(ctx, base_id, post),
        "type_is" => ingest_type_is_construct(ctx, base_id, post, false),
        "type_is_exactly" => ingest_type_is_construct(ctx, base_id, post, true),
        "info" => ingest_info_construct(ctx, base_id),
        other => todo!("Nice error {}", other),
    }
}

fn ingest_type_is_construct(
    ctx: &mut Context,
    base_id: ItemId,
    post: Construct,
    exact: bool,
) -> Result<UnresolvedItem, String> {
    let label = if exact { "type_is_exactly" } else { "type_is" };
    let typee_expr = post.expect_single_expression(label).unwrap();
    let typee = ingest_expression(&mut ctx.child(), typee_expr.clone(), Default::default())?;
    Ok(Item::TypeIs {
        base_type_only: exact,
        base: base_id,
        typee,
    }
    .into())
}

fn ingest_info_construct(ctx: &mut Context, base_id: ItemId) -> Result<UnresolvedItem, String> {
    let scope = ctx.defined_in;
    ctx.environment.mark_info(base_id, scope);
    Ok(UnresolvedItem::Item(base_id))
}

fn ingest_replacing_construct(
    ctx: &mut Context,
    base_id: ItemId,
    post: Construct,
) -> Result<UnresolvedItem, String> {
    let statements = post.expect_statements("replacing")?.to_owned();
    let (replacements, unlabeled_replacements) = ingest_replacements(ctx, statements)?;
    if unlabeled_replacements.len() != 0 {
        todo!()
    }
    Ok(Item::Replacing {
        base: base_id,
        replacements,
    }
    .into())
}

fn ingest_member_construct(base_id: ItemId, post: Construct) -> Result<UnresolvedItem, String> {
    let name = post
        .expect_single_expression("member")?
        .clone()
        .expect_ident_owned()?;
    Ok(UnresolvedItem::Member {
        base: base_id,
        name,
    })
}
