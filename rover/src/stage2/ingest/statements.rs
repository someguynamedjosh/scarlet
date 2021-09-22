use super::context::LocalInfo;
use crate::{
    stage1::structure::{expression::Expression, statement::Statement},
    stage2::{
        ingest::{context::Context, expression::ingest_expression},
        structure::{Definitions, ItemId, Replacements},
    },
};

struct UnprocessedItem {
    id: ItemId,
    public: bool,
    name: String,
    def: Expression,
}

pub(super) fn process_definitions(
    ctx: &mut Context,
    statements: Vec<Statement>,
    other_defs: Vec<(String, ItemId)>,
) -> Result<Definitions, String> {
    process_definitions_with_info(ctx, statements, other_defs, LocalInfo::Plain)
}

pub(super) fn process_definitions_with_info(
    ctx: &mut Context,
    statements: Vec<Statement>,
    other_defs: Vec<(String, ItemId)>,
    info: LocalInfo,
) -> Result<Definitions, String> {
    let mut top_level_expressions = Vec::new();
    for statement in statements {
        match statement {
            Statement::Is(is) => {
                let name = is.name.expect_ident_owned()?;
                top_level_expressions.push(UnprocessedItem {
                    id: ctx.environment.next_id(),
                    public: is.public,
                    name,
                    def: is.value,
                });
            }
            Statement::Else(..)
            | Statement::Expression(..)
            | Statement::PickElif(..)
            | Statement::PickIf(..)
            | Statement::Replace(..) => todo!("nice error"),
        }
    }
    let definitions: Vec<_> = other_defs
        .into_iter()
        .chain(top_level_expressions.iter().map(|i| (i.name.clone(), i.id)))
        .collect();
    for item in top_level_expressions {
        let mut child_ctx = ctx
            .child()
            .with_current_item_id(item.id)
            .with_additional_scope(&definitions)
            .with_local_info(info.clone());
        ingest_expression(&mut child_ctx, item.def)?;
    }
    Ok(definitions)
}

pub(super) fn process_replacements(
    ctx: &mut Context,
    statements: Vec<Statement>,
) -> Result<(Replacements, Vec<ItemId>), String> {
    let mut replacements = Replacements::new();
    let mut unlabeled_replacements = Vec::new();
    for statement in statements {
        match statement {
            Statement::Is(..) => todo!("nice error"),
            Statement::Replace(s) => {
                let target = ingest_expression(&mut ctx.child(), s.target)?;
                let value = ingest_expression(&mut ctx.child(), s.value)?;
                replacements.push((target, value));
            }
            Statement::Expression(e) => {
                let value = ingest_expression(&mut ctx.child(), e)?;
                unlabeled_replacements.push(value);
            }
            Statement::Else(..) | Statement::PickElif(..) | Statement::PickIf(..) => {
                todo!("nice error")
            }
        }
    }
    Ok((replacements, unlabeled_replacements))
}
