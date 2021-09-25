use crate::{
    shared::{Definitions, Item, ItemId, PrimitiveValue},
    stage1::structure::{construct::Construct, statement::Statement},
    stage2::{
        ingest::{
            context::{Context, LocalInfo},
            definitions::process_definitions_with_info,
            expression::ingest_expression,
        },
        structure::UnresolvedItem,
    },
};

fn type_self(ctx: &mut Context) -> (ItemId, (String, ItemId)) {
    let self_id = ctx.get_or_create_current_id();
    let self_def = ("Self".to_string(), self_id);
    (self_id, self_def)
}

pub fn ingest_type_construct(ctx: &mut Context, root: Construct) -> Result<UnresolvedItem, String> {
    let statements = root.expect_statements("Type").unwrap().to_owned();
    let mut params = Vec::new();
    for statement in &statements {
        match statement {
            Statement::Parameter(s) => params.push((s.clone(), ctx.environment.next_id())),
            _ => (),
        }
    }

    let (self_id, self_def) = type_self(ctx);
    let inner_type = Item::InductiveType {
        selff: self_id,
        params: params.iter().map(|i| i.1).collect(),
    };
    let inner_type_id = ctx.environment.insert_item(inner_type);
    ctx.environment.set_defined_in(inner_type_id, self_id);

    let info = LocalInfo::Type(self_id);
    let definitions =
        process_definitions_with_info(ctx, statements, vec![self_def], info, self_id)?;

    let this_id = ctx.get_or_create_current_id();
    for (statement, into_id) in params {
        ctx.environment.set_defined_in(into_id, this_id);
        let mut this_ctx = ctx
            .child()
            .with_additional_scope(&definitions)
            .with_current_item_id(into_id);
        ingest_expression(&mut this_ctx, statement.0, vec![])?;
    }

    Ok(Item::Defining {
        base: inner_type_id,
        definitions,
    }
    .into())
}

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
    let typee = ingest_expression(&mut ctx.child(), typ_expr, vec![])?;
    let selff = ctx.get_or_create_current_id();
    Ok(Item::Variable { selff, typee }.into())
}

pub fn ingest_i32_construct(root: Construct) -> Result<UnresolvedItem, String> {
    let val = root.expect_text("i32")?;
    let val: i32 = val.parse().unwrap();
    Ok(Item::PrimitiveValue(PrimitiveValue::I32(val)).into())
}
