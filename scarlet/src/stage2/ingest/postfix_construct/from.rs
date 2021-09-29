use crate::{
    shared::{Item, ItemId, VarList},
    stage1::structure::{construct::Construct, expression::Expression, statement::Statement},
    stage2::{
        ingest::{context::Context, expression::ingest_expression},
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
        vars: values,
    }
    .into();
    Ok(item)
}

fn ingest_from_statements(ctx: &mut Context, from: Construct) -> Result<VarList, String> {
    let statements = from.expect_statements("From").unwrap().to_owned();
    let mut values = VarList::new();
    for statement in statements {
        ingest_from_statement(&mut ctx.child(), &mut values, statement)?
    }
    Ok(values)
}

fn ingest_from_statement(
    ctx: &mut Context,
    vars: &mut VarList,
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
    vars: &mut VarList,
    var_expr: Expression,
) -> Result<(), String> {
    let var_id = ingest_expression(ctx, var_expr, Default::default())?;
    vars.push(var_id);
    Ok(())
}
