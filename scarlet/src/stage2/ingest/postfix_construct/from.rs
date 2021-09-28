use crate::{
    shared::{Definitions, Item, ItemId},
    stage1::structure::{
        construct::Construct,
        expression::Expression,
        statement::{Is, Statement},
    },
    stage2::{
        ingest::{context::Context, expression::ingest_expression, helpers::with_definitions},
        structure::UnresolvedItem,
    },
};

pub fn ingest_from_construct(
    ctx: &mut Context,
    base_id: ItemId,
    from: Construct,
) -> Result<UnresolvedItem, String> {
    let values = ingest_from_statements(ctx, from)?;
    let item = Item::FromType {
        base: base_id,
        values,
    }
    .into();
    Ok(item)
}

fn ingest_from_statements(ctx: &mut Context, from: Construct) -> Result<Vec<ItemId>, String> {
    let statements = from.expect_statements("From").unwrap().to_owned();
    let mut values = Vec::new();
    for statement in statements {
        ingest_from_statement(&mut ctx.child(), &mut values, statement)?
    }
    Ok(values)
}

fn ingest_from_statement(
    ctx: &mut Context,
    vars: &mut Vec<ItemId>,
    statement: Statement,
) -> Result<(), String> {
    match statement {
        Statement::Expression(expr) => ingest_from_variable(ctx, vars, expr),
        Statement::Is(..) => todo!(),
        _ => todo!("nice error"),
    }
}

fn ingest_from_variable(
    ctx: &mut Context,
    vars: &mut Vec<ItemId>,
    var_expr: Expression,
) -> Result<(), String> {
    let var_id = ingest_expression(ctx, var_expr, Default::default())?;
    vars.push(var_id);
    Ok(())
}
