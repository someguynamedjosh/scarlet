use super::helpers;
use crate::{
    stage1::structure::{expression::Expression, statement::Statement},
    stage2::structure::{Environment, ItemId},
};

pub fn vomit(env: &Environment, target: ItemId, value: ItemId, base: ItemId) -> Expression {
    let statement = build_statement(env, target, value);
    build_replacing_expr(env, vec![statement], base)
}

fn build_statement(env: &Environment, target: ItemId, value: ItemId) -> Statement {
    let target = super::vomit(env, target);
    let mut value = super::vomit(env, value);
    value
        .pres
        .push(helpers::single_expr_construct("target", target));
    Statement::Expression(value)
}

fn build_replacing_expr(env: &Environment, statements: Vec<Statement>, base: ItemId) -> Expression {
    let substituting = helpers::statements_construct("substituting", statements);
    let mut base = super::vomit(env, base);
    base.posts.push(substituting);
    base
}
