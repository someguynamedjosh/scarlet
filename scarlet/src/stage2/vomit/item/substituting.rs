use super::helpers;
use crate::{
    stage1::structure::expression::Expression,
    stage2::structure::{Environment, ItemId, Substitutions},
};

pub fn vomit(env: &Environment, substitutions: &Substitutions, base: ItemId) -> Expression {
    let mut expressions = Vec::new();
    for (target, value) in substitutions {
        let mut value = super::vomit(env, *value);
        if let Some(target) = target {
            let target = super::vomit(env, *target);
            let target = helpers::single_expr_construct("target", target);
            value.pres.push(target);
        }
        expressions.push(value);
    }
    build_replacing_expr(env, expressions, base)
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
