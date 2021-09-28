use crate::{
    shared::{Definitions, ItemId},
    stage1::structure::{
        expression::Expression,
        statement::{Is, Statement},
    },
    stage2::ingest::{context::Context, expression::ingest_expression},
};

struct UnprocessedItem {
    id: ItemId,
    name: Option<String>,
    def: Expression,
}

pub(super) fn process_definitions(
    ctx: &mut Context,
    statements: Vec<Statement>,
    other_defs: Definitions,
    self_id: ItemId,
) -> Result<Definitions, String> {
    let unprocessed = statements_to_unprocessed_items(ctx, statements)?;
    let definitions = definitions(other_defs, &unprocessed[..]);
    for item in unprocessed {
        let mut child_ctx = ctx.child().with_id_and_scope(item.id, &definitions);
        let id = ingest_expression(&mut child_ctx, item.def, Default::default())?;
        ctx.environment.set_defined_in(id, self_id);
    }
    Ok(definitions)
}

fn is_statement_to_unprocessed_item(ctx: &mut Context, is: Is) -> Result<UnprocessedItem, String> {
    let name = is.name.expect_ident_owned()?;
    Ok(UnprocessedItem {
        id: ctx.environment.next_id(),
        name: Some(name),
        def: is.value,
    })
}

fn expr_statement_to_unprocessed_item(
    ctx: &mut Context,
    expr: Expression,
) -> Result<UnprocessedItem, String> {
    Ok(UnprocessedItem {
        id: ctx.environment.next_id(),
        name: None,
        def: expr,
    })
}

fn statements_to_unprocessed_items(
    ctx: &mut Context,
    statements: Vec<Statement>,
) -> Result<Vec<UnprocessedItem>, String> {
    let mut top_level_expressions = Vec::new();
    for statement in statements {
        let item = match statement {
            Statement::Is(is) => is_statement_to_unprocessed_item(ctx, is)?,
            Statement::Expression(expr) => expr_statement_to_unprocessed_item(ctx, expr)?,
            Statement::Parameter(..) => continue,
            _ => todo!("Nice error"),
        };
        top_level_expressions.push(item);
    }
    Ok(top_level_expressions)
}

fn item_to_def(item: &UnprocessedItem) -> Option<(String, ItemId)> {
    item.name.as_ref().map(|name| (name.clone(), item.id))
}

fn definitions(other_defs: Definitions, unprocessed: &[UnprocessedItem]) -> Definitions {
    let unprocessed_defs = unprocessed.iter().filter_map(item_to_def).collect();
    other_defs.after_inserting(&unprocessed_defs)
}
