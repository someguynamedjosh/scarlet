use crate::{
    shared::{Definitions, Item, ItemId, PrimitiveValue},
    stage1::structure::construct::Construct,
    stage2::{
        ingest::{context::Context, expression::ingest_expression},
        structure::UnresolvedItem,
    },
};

fn resolve_ident_in_scope(scope: &Definitions, ident: &str) -> Option<ItemId> {
    for (name, val) in scope {
        if name == ident {
            return Some(*val);
        }
    }
    None
}

fn resolve_ident(ctx: &Context, ident: &str) -> Result<ItemId, String> {
    // Reverse to earch the closest parents first.
    for scope in ctx.parent_scopes.iter().rev() {
        println!("{:?}", scope);
        if let Some(id) = resolve_ident_in_scope(scope, ident) {
            return Ok(id);
        }
    }
    Err(format!(
        "Could not find an item named {} in the current scope or its parents.",
        ident
    ))
}

pub fn ingest_identifier(ctx: &mut Context, root: Construct) -> Result<UnresolvedItem, String> {
    let ident = root.expect_ident()?;
    let resolved = resolve_ident(ctx, ident)?;
    Ok(UnresolvedItem::Item(resolved))
}

pub fn ingest_any_construct(ctx: &mut Context, root: Construct) -> Result<UnresolvedItem, String> {
    let typ_expr = root.expect_single_expression("any")?.clone();
    let typee = ingest_expression(&mut ctx.child(), typ_expr, Default::default())?;
    let selff = ctx.get_or_create_current_id();
    Ok(Item::Variable { selff, typee }.into())
}

pub fn ingest_i32_construct(root: Construct) -> Result<UnresolvedItem, String> {
    let val = root.expect_text("i32")?;
    let val: i32 = val.parse().unwrap();
    Ok(Item::PrimitiveValue(PrimitiveValue::I32(val)).into())
}
