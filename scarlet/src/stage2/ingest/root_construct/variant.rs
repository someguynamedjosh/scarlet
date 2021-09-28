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

fn dereference_type(ctx: &Context, type_id: ItemId) -> ItemId {
    match &ctx.environment.definition_of(type_id).definition {
        Some(UnresolvedItem::Just(item)) => match item {
            Item::Defining { base, .. } | Item::FromType { base, .. } => {
                dereference_type(ctx, *base)
            }
            _ => type_id,
        },
        _ => type_id,
    }
}

fn get_from_vars(ctx: &Context, type_id: ItemId) -> (Vec<ItemId>, Definitions) {
    match &ctx.environment.definition_of(type_id).definition {
        Some(UnresolvedItem::Just(item)) => match item {
            Item::FromType { base, values: vars } => {
                let (base_vars, defs) = get_from_vars(ctx, *base);
                let vars = [base_vars, vars.clone()].concat();
                (vars, defs)
            }
            Item::Defining { base, definitions } => {
                let (vars, base_defs) = get_from_vars(ctx, *base);
                let defs = base_defs.after_inserting(definitions);
                (vars, defs)
            }
            _ => (Default::default(), Default::default()),
        },
        _ => (Default::default(), Default::default()),
    }
}

pub fn ingest_variant_construct(
    ctx: &mut Context,
    root: Construct,
) -> Result<UnresolvedItem, String> {
    let type_expr = get_variant_type(root)?;
    let return_type_id = ingest_expression(&mut ctx.child(), type_expr, Default::default())?;

    let base_return_type_id = dereference_type(ctx, return_type_id);
    let (paramed_vars, definitions) = get_from_vars(ctx, return_type_id);
    let variant_id = ctx.get_or_create_current_id();
    for var in &paramed_vars {
        ctx.environment.set_defined_in(*var, variant_id);
    }

    let val = Item::VariantInstance {
        typee: base_return_type_id,
        variant_id,
        values: paramed_vars,
    }
    .into();
    if definitions.len() > 0 {
        let val_id = ctx.environment.insert_unresolved_item(val);
        ctx.environment.set_defined_in(val_id, variant_id);
        Ok(Item::Defining {
            base: val_id,
            definitions,
        }
        .into())
    } else {
        Ok(val)
    }
}
