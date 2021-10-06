use super::helpers;
use crate::{
    stage1::structure::{
        expression::Expression,
        statement::{Is, Statement},
    },
    stage2::structure::{Environment, Item, ItemId},
};

pub fn vomit(env: &Environment, target: ItemId, value: ItemId, base: ItemId) -> Expression {
    let statement = build_statement(env, target, value);
    build_replacing_expr(env, vec![statement], base)
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
    base.posts.push(substituting);
    base
}
