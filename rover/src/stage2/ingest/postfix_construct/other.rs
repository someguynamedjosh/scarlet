use crate::{
    stage1::structure::construct::Construct,
    stage2::{
        ingest::{
            context::Context, expression::ingest_expression,
            postfix_construct::from::ingest_from_construct, replacements::ingest_replacements,
        },
        structure::{Item, ItemId},
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
    Ok(Item::IsSameVariant {
        base: base_id,
        other,
    })
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
    Ok(Item::TypeIs {
        exact,
        base: base_id,
        typee,
    })
}

fn ingest_replacing_construct(
    ctx: &mut Context,
    base_id: ItemId,
    post: Construct,
) -> Result<Item, String> {
    let statements = post.expect_statements("replacing")?.to_owned();
    let (replacements, unlabeled_replacements) = ingest_replacements(&mut ctx.child(), statements)?;
    Ok(Item::Replacing {
        base: base_id,
        replacements,
        unlabeled_replacements,
    })
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
