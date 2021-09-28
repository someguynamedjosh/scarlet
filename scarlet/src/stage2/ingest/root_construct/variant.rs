use crate::{
    shared::{Definitions, Item, ItemId},
    stage1::structure::{construct::Construct, expression::Expression},
    stage2::{
        ingest::{context::Context, expression::ingest_expression},
        structure::UnresolvedItem,
    },
};

fn get_variant_type(root: Construct) -> Result<Expression, String> {
    let type_expr = root.expect_single_expression("variant")?;
    Ok(type_expr.clone())
}

pub fn ingest_variant_construct(
    ctx: &mut Context,
    root: Construct,
) -> Result<UnresolvedItem, String> {
    let type_expr = get_variant_type(root)?;
    let return_type_id = ingest_expression(&mut ctx.child(), type_expr, Default::default())?;
    let variant_id = ctx.get_or_create_current_id();

    let val = Item::Variant {
        typee: return_type_id,
        selff: variant_id,
    }
    .into();
    Ok(val)
}
