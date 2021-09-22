use crate::{
    shared::{ItemId, Replacements},
    stage1::structure::{
        expression::Expression,
        statement::{Replace, Statement},
    },
    stage2::ingest::{context::Context, expression::ingest_expression},
};

fn ingest_labeled_replacement(
    ctx: &mut Context,
    replacements: &mut Replacements,
    replacement: Replace,
) -> Result<(), String> {
    let target = ingest_expression(&mut ctx.child(), replacement.target)?;
    let value = ingest_expression(&mut ctx.child(), replacement.value)?;
    replacements.push((target, value));
    Ok(())
}

fn ingest_unlabeled_replacement(
    ctx: &mut Context,
    unlabeled_replacements: &mut Vec<ItemId>,
    replacement: Expression,
) -> Result<(), String> {
    let value = ingest_expression(&mut ctx.child(), replacement)?;
    unlabeled_replacements.push(value);
    Ok(())
}

fn ingest_replacement(
    ctx: &mut Context,
    replacements: &mut Replacements,
    unlabeled_replacements: &mut Vec<ItemId>,
    replacement: Statement,
) -> Result<(), String> {
    match replacement {
        Statement::Replace(replacement) => {
            ingest_labeled_replacement(ctx, replacements, replacement)
        }
        Statement::Expression(replacement) => {
            ingest_unlabeled_replacement(ctx, unlabeled_replacements, replacement)
        }
        _ => {
            todo!("nice error")
        }
    }
}

pub(super) fn ingest_replacements(
    ctx: &mut Context,
    statements: Vec<Statement>,
) -> Result<(Replacements, Vec<ItemId>), String> {
    let mut replacements = Replacements::new();
    let mut unlabeled_replacements = Vec::new();
    for replacement in statements {
        ingest_replacement(
            ctx,
            &mut replacements,
            &mut unlabeled_replacements,
            replacement,
        )?;
    }
    Ok((replacements, unlabeled_replacements))
}
