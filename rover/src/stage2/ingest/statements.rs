use crate::{
    parse::statements::Statement,
    stage2::{
        helpers::{expect_ident_expr, Context, UnprocessedItem},
        ingest::expression::process_expr,
        structure::{Definitions, Environment, ItemId, Replacements},
    },
};

pub(super) fn process_definitions(
    statements: Vec<Statement>,
    other_defs: Vec<(String, ItemId)>,
    env: &mut Environment,
    ctx: Context,
    parents: &[&Definitions],
) -> Result<Definitions, String> {
    let mut top_level_expressions = Vec::new();
    for statement in statements {
        match statement {
            Statement::Is(is) => {
                let name = expect_ident_expr(is.name)?;
                top_level_expressions.push(UnprocessedItem {
                    id: env.next_id(),
                    public: is.public,
                    name,
                    def: is.value,
                });
            }
            Statement::Replace(s) => todo!("nice error"),
            Statement::Expression(..) => todo!("Nice error"),
        }
    }
    let definitions: Vec<_> = other_defs
        .into_iter()
        .chain(top_level_expressions.iter().map(|i| (i.name.clone(), i.id)))
        .collect();
    let parents: Vec<_> = parents
        .iter()
        .copied()
        .chain(std::iter::once(&definitions))
        .collect();
    let parents = &parents[..];
    for item in top_level_expressions {
        let next_ctx = match &ctx {
            Context::Type(type_item) => Context::TypeMember(*type_item, item.name.clone()),
            _ => Context::Plain,
        };
        process_expr(item.def, Some(item.id), env, next_ctx, parents)?;
    }
    Ok(definitions)
}

pub(super) fn process_replacements(
    statements: Vec<Statement>,
    env: &mut Environment,
    parents: &[&Definitions],
) -> Result<Replacements, String> {
    let mut replacements = Replacements::new();
    for statement in statements {
        match statement {
            Statement::Is(..) => todo!("nice error"),
            Statement::Replace(s) => {
                let ctx = Context::Plain;
                let target = process_expr(s.target, None, env, ctx, parents)?;
                let ctx = Context::Plain;
                let value = process_expr(s.value, None, env, ctx, parents)?;
                replacements.push((target, value));
            }
            Statement::Expression(..) => todo!("Nice error"),
        }
    }
    Ok(replacements)
}
