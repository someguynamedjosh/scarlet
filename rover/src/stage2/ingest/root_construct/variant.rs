use crate::{
    shared::{Item, ItemId},
    stage1::structure::{construct::Construct, expression::Expression},
    stage2::{
        ingest::{
            context::{Context, LocalInfo},
            expression::ingest_expression,
        },
        structure::UnresolvedItem,
    },
};

fn check_containing_type(ctx: &Context, expected_type: ItemId) -> Result<(), String> {
    if let LocalInfo::Type(typee) = ctx.local_info {
        if expected_type == typee {
            Ok(())
        } else {
            todo!("nice error, variant type is not Self.")
        }
    } else {
        todo!("nice error, not in a type.")
    }
}

fn decompose_variant_construct(root: Construct) -> Result<(String, Expression), String> {
    let def_expr = root.expect_single_expression("variant")?;
    let variant_name = def_expr.root.expect_ident()?.to_owned();
    if def_expr.others.len() != 1 {
        todo!("nice error");
    }
    let type_expr = def_expr.others[0]
        .expect_single_expression("type_is")?
        .clone();
    Ok((variant_name, type_expr))
}

fn dereference_type(ctx: &Context, type_id: ItemId) -> ItemId {
    match ctx.environment.definition_of(type_id) {
        Some(UnresolvedItem::Just(item)) => match item {
            Item::Defining { base, .. } | Item::FromType { base, .. } => {
                dereference_type(ctx, *base)
            }
            _ => type_id,
        },
        _ => type_id,
    }
}

fn get_from_vars(ctx: &Context, type_id: ItemId) -> Vec<ItemId> {
    match ctx.environment.definition_of(type_id) {
        Some(UnresolvedItem::Just(Item::FromType { base, vars })) => {
            let base_vars = get_from_vars(ctx, *base);
            [base_vars, vars.clone()].concat()
        }
        _ => vec![],
    }
}

pub fn ingest_variant_construct(
    ctx: &mut Context,
    root: Construct,
) -> Result<UnresolvedItem, String> {
    let (variant_name, type_expr) = decompose_variant_construct(root)?;
    let return_type_id = ingest_expression(&mut ctx.child(), type_expr)?;

    let base_return_type_id = dereference_type(ctx, return_type_id);
    check_containing_type(ctx, base_return_type_id)?;
    let recorded_vars = get_from_vars(ctx, return_type_id);

    Ok(Item::InductiveValue {
        typee: base_return_type_id,
        variant_name,
        records: recorded_vars,
    }
    .into())
}
