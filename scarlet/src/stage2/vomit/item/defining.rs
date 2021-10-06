use super::helpers;
use crate::{
    stage1::structure::{expression::Expression, statement::Statement},
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
        let mut value = super::vomit(env, *value);
        value
            .pres
            .push(helpers::single_expr_construct("target", name));
        statements.push(Statement::Expression(value));
    }
    statements
}

fn build_expr(env: &Environment, statements: Vec<Statement>, base: ItemId) -> Expression {
    let construct = helpers::statements_construct("defining", statements);
    let mut expr = super::vomit(env, base);
    expr.pres.insert(0, construct);
    expr
}
