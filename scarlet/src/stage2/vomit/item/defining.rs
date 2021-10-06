use super::helpers;
use crate::{
    stage1::structure::{
        expression::Expression,
        statement::{Is, Statement},
    },
    stage2::structure::{Definitions, Environment, Item, ItemId},
};

pub fn vomit(env: &Environment, definitions: &Definitions, base: ItemId) -> Expression {
    let statements = build_statements(env, definitions);
    let expr = build_expr(env, statements, base);
    expr
}

fn build_statements(env: &Environment, definitions: &Definitions) -> Vec<Statement> {
    let mut statements = Vec::new();
    for (name, value) in definitions {
        let name = helpers::just_root_expression(helpers::identifier(name));
        let value = super::vomit(env, *value);
        let statement = Is { name, value };
        statements.push(Statement::Is(statement));
    }
    statements
}

fn build_expr(env: &Environment, statements: Vec<Statement>, base: ItemId) -> Expression {
    let construct = helpers::statements_construct("defining", statements);
    let mut expr = super::vomit(env, base);
    expr.posts.push(construct);
    expr
}
