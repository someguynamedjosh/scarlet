use crate::{
    shared::{ItemId, ResolvedItem},
    stage1::structure::construct::Construct,
    stage2::{
        ingest::{
            context::Context, expression::ingest_expression,
            postfix_construct::from::ingest_from_construct, replacements::ingest_replacements,
        },
        structure::Item,
    },
};

pub fn ingest_non_defining_postfix_construct(
    ctx: &mut Context,
    base_id: ItemId,
    post: Construct,
) -> Result<Item, String> {
    match &post.label[..] {
        "replacing" => ingest_replacing_construct(ctx, base_id, post),
        "member" => ingest_member_construct(base_id, post),
        "From" => ingest_from_construct(ctx, base_id, post),
        "is_variant" => ingest_is_variant_construct(ctx, base_id, post),
        "type_is" => ingest_type_is_construct(ctx, base_id, post, false),
        "type_is_exactly" => ingest_type_is_construct(ctx, base_id, post, true),
        _ => unreachable!(),
    }
}

fn ingest_is_variant_construct(
    ctx: &mut Context,
    base_id: ItemId,
    post: Construct,
) -> Result<Item, String> {
    let other = post.expect_single_expression("is_variant")?;
    let other = ingest_expression(&mut ctx.child(), other.clone())?;
    Ok(ResolvedItem::IsSameVariant {
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
) -> Result<Item, String> {
    let label = if exact { "type_is_exactly" } else { "type_is" };
    let typee_expr = post.expect_single_expression(label).unwrap();
    let typee = ingest_expression(&mut ctx.child(), typee_expr.clone())?;
    Ok(ResolvedItem::TypeIs {
        exact,
        base: base_id,
        typee,
    }
    .into())
}

fn ingest_replacing_construct(
    ctx: &mut Context,
    base_id: ItemId,
    post: Construct,
) -> Result<Item, String> {
    let statements = post.expect_statements("replacing")?.to_owned();
    let (replacements, unlabeled_replacements) = ingest_replacements(&mut ctx.child(), statements)?;
    Ok(ResolvedItem::Replacing {
        base: base_id,
        replacements,
        unlabeled_replacements,
    }
    .into())
}

fn ingest_member_construct(base_id: ItemId, post: Construct) -> Result<Item, String> {
    let name = post
        .expect_single_expression("member")?
        .clone()
        .expect_ident_owned()?;
    Ok(Item::Member {
        base: base_id,
        name,
    })
}
