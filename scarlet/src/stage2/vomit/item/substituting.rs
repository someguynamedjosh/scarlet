use super::helpers;
use crate::{
    stage1::structure::expression::Expression,
    stage2::structure::{Environment, ItemId},
};

pub fn vomit(env: &Environment, target: ItemId, value: ItemId, base: ItemId) -> Expression {
    let expression = build_expression(env, target, value);
    build_replacing_expr(env, vec![expression], base)
}

fn build_expression(env: &Environment, target: ItemId, value: ItemId) -> Expression {
    let target = super::vomit(env, target);
    let target = helpers::single_expr_construct("target", target);
    let mut value = super::vomit(env, value);
    value.pres.push(target);
    value
}

fn build_replacing_expr(
    env: &Environment,
    expressions: Vec<Expression>,
    base: ItemId,
) -> Expression {
    let substituting = helpers::expressions_construct("substituting", expressions);
    let mut base = super::vomit(env, base);
    base.posts.push(substituting);
    base
}
