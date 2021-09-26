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
        "is_variant" => ingest_is_variant_construct(ctx, base_id, post),
        "type_is" => ingest_type_is_construct(ctx, base_id, post, false),
        "type_is_exactly" => ingest_type_is_construct(ctx, base_id, post, true),
        "info" => ingest_info_construct(ctx, base_id),
        _ => todo!("Nice error"),
    }
}

fn ingest_is_variant_construct(
    ctx: &mut Context,
    base_id: ItemId,
    post: Construct,
) -> Result<UnresolvedItem, String> {
    let other = post.expect_single_expression("is_variant")?;
    let other = ingest_expression(&mut ctx.child(), other.clone(), vec![])?;
    Ok(Item::IsSameVariant {
        base: base_id,
        other,
    }
    .into())
}

fn ingest_type_is_construct(
    ctx: &mut Context,
    base_id: ItemId,
    post: Construct,
    exact: bool,
) -> Result<UnresolvedItem, String> {
    let label = if exact { "type_is_exactly" } else { "type_is" };
    let typee_expr = post.expect_single_expression(label).unwrap();
    let typee = ingest_expression(&mut ctx.child(), typee_expr.clone(), vec![])?;
    Ok(Item::TypeIs {
        exact,
        base: base_id,
        typee,
    }
    .into())
}

fn ingest_info_construct(ctx: &mut Context, base_id: ItemId) -> Result<UnresolvedItem, String> {
    ctx.environment.mark_info(base_id);
    Ok(UnresolvedItem::Item(base_id))
}

fn ingest_replacing_construct(
    ctx: &mut Context,
    base_id: ItemId,
    post: Construct,
) -> Result<UnresolvedItem, String> {
    let statements = post.expect_statements("replacing")?.to_owned();
    let (replacements, unlabeled_replacements) = ingest_replacements(&mut ctx.child(), statements)?;
    Ok(Item::Replacing {
        base: base_id,
        replacements,
        unlabeled_replacements,
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
