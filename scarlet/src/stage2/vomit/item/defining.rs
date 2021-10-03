use super::helpers;
use crate::{
    stage1::structure::{
        expression::Expression,
        statement::{Is, Statement},
    },
    stage2::structure::{Definitions, Item},
};

pub fn vomit(definitions: &Definitions, base: &Item) -> Expression {
    let statements = build_statements(definitions);
    let expr = build_expr(statements, base);
    expr
}

fn build_statements(definitions: &Definitions) -> Vec<Statement> {
    let mut statements = Vec::new();
    for (name, value) in definitions {
        let name = helpers::just_root_expression(helpers::identifier(name));
        let value = super::vomit(value);
        let statement = Is { name, value };
        statements.push(Statement::Is(statement));
    }
    statements
}

fn build_expr(statements: Vec<Statement>, base: &Item) -> Expression {
    let construct = helpers::statements_construct("defining", statements);
    let mut expr = super::vomit(base);
    expr.others.push(construct);
    expr
}
