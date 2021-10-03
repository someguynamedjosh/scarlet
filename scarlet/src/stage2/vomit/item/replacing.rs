use super::helpers;
use crate::{
    stage1::structure::{
        expression::Expression,
        statement::{Replace, Statement},
    },
    stage2::structure::{Item, Replacements},
};

pub fn vomit(replacements: &Replacements, base: &Item) -> Expression {
    let statements = build_statements(replacements);
    build_replacing_expr(statements, base)
}

fn build_statements(replacements: &Replacements) -> Vec<Statement> {
    let mut statements = Vec::new();
    for (target, value) in replacements {
        statements.push(build_statement(target, value));
    }
    statements
}

fn build_statement(target: &Item, value: &Item) -> Statement {
    let target = super::vomit(target);
    let value = super::vomit(value);
    let statement = Replace { target, value };
    let statement = Statement::Replace(statement);
    statement
}

fn build_replacing_expr(statements: Vec<Statement>, base: &Item) -> Expression {
    let replacing = helpers::statements_construct("replacing", statements);
    let mut base = super::vomit(base);
    base.others.push(replacing);
    base
}
