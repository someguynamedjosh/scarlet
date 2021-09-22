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
    let (vars, named_vars) = ingest_from_statements(ctx, from)?;
    let base_item = Item::FromType {
        base: base_id,
        vars,
    }
    .into();
    Ok(with_definitions(ctx, base_item, named_vars))
}

fn ingest_from_statements(
    ctx: &mut Context,
    from: Construct,
) -> Result<(Vec<ItemId>, Definitions), String> {
    let statements = from.expect_statements("From").unwrap().to_owned();
    let mut vars = Vec::new();
    let mut named_vars = Vec::new();
    for statement in statements {
        ingest_from_statement(ctx, &mut vars, &mut named_vars, statement)?
    }
    Ok((vars, named_vars))
}

fn ingest_from_statement(
    ctx: &mut Context,
    vars: &mut Vec<ItemId>,
    named_vars: &mut Definitions,
    statement: Statement,
) -> Result<(), String> {
    match statement {
        Statement::Expression(expr) => ingest_from_variable(ctx, vars, expr),
        Statement::Is(is) => ingest_from_definition(ctx, vars, named_vars, is),
        Statement::Else(..)
        | Statement::PickElif(..)
        | Statement::PickIf(..)
        | Statement::Replace(..) => todo!("nice error"),
    }
}

fn ingest_from_variable(
    ctx: &mut Context,
    vars: &mut Vec<ItemId>,
    var_expr: Expression,
) -> Result<(), String> {
    let var_id = ingest_expression(&mut ctx.child(), var_expr)?;
    vars.push(var_id);
    Ok(())
}

fn ingest_from_definition(
    ctx: &mut Context,
    vars: &mut Vec<ItemId>,
    named_vars: &mut Definitions,
    definition: Is,
) -> Result<(), String> {
    let name = definition.name.expect_ident_owned()?;
    let expr = definition.value;
    let var = ingest_expression(&mut ctx.child(), expr)?;
    named_vars.push((name, var));
    vars.push(var);
    Ok(())
}
