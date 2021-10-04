use super::helpers;
use crate::{
    stage1::structure::{
        expression::Expression,
        statement::{Is, Statement},
    },
    stage2::structure::{Environment, Item, ItemId, Substitutions},
};

pub fn vomit(env: &Environment, substitutions: &Substitutions, base: ItemId) -> Expression {
    let statements = build_statements(env, substitutions);
    build_replacing_expr(env, statements, base)
}

fn build_statements(env: &Environment, substitutions: &Substitutions) -> Vec<Statement> {
    let mut statements = Vec::new();
    for (target, value) in substitutions {
        statements.push(build_statement(env, *target, *value));
    }
    statements
}

fn build_statement(env: &Environment, target: ItemId, value: ItemId) -> Statement {
    let target = super::vomit(env, target);
    let value = super::vomit(env, value);
    let statement = Is {
        name: target,
        value,
    };
    let statement = Statement::Is(statement);
    statement
}

fn build_replacing_expr(env: &Environment, statements: Vec<Statement>, base: ItemId) -> Expression {
    let substituting = helpers::statements_construct("substituting", statements);
    let mut base = super::vomit(env, base);
    base.others.push(substituting);
    base
}
