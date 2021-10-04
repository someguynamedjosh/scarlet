use super::helpers;
use crate::{
    stage1::structure::{
        expression::Expression,
        statement::{Is, Statement},
    },
    stage2::structure::{Item, Substitutions},
};

pub fn vomit(substitutions: &Substitutions, base: &Item) -> Expression {
    let statements = build_statements(substitutions);
    build_replacing_expr(statements, base)
}

fn build_statements(substitutions: &Substitutions) -> Vec<Statement> {
    let mut statements = Vec::new();
    for (target, value) in substitutions {
        statements.push(build_statement(target, value));
    }
    statements
}

fn build_statement(target: &Item, value: &Item) -> Statement {
    let target = super::vomit(target);
    let value = super::vomit(value);
    let statement = Is {
        name: target,
        value,
    };
    let statement = Statement::Is(statement);
    statement
}

fn build_replacing_expr(statements: Vec<Statement>, base: &Item) -> Expression {
    let substituting = helpers::statements_construct("substituting", statements);
    let mut base = super::vomit(base);
    base.others.push(substituting);
    base
}
