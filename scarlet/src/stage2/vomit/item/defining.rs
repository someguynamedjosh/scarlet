use super::helpers;
use crate::{
    stage1::structure::expression::Expression,
    stage2::structure::{Definitions, Environment, ItemId},
};

pub fn vomit(env: &Environment, definitions: &Definitions, base: ItemId) -> Expression {
    let expressions = build_expressions(env, definitions);
    let expr = build_expr(env, expressions, base);
    expr
}

fn build_expressions(env: &Environment, definitions: &Definitions) -> Vec<Expression> {
    let mut expressions = Vec::new();
    for (name, value) in definitions {
        let name = helpers::just_root_expression(helpers::identifier(name));
        let target = helpers::single_expr_construct("target", name);
        let mut value = super::vomit(env, *value);
        value.pres.push(target);
        expressions.push(value);
    }
    expressions
}

fn build_expr(env: &Environment, expressions: Vec<Expression>, base: ItemId) -> Expression {
    let construct = helpers::expressions_construct("defining", expressions);
    let mut expr = super::vomit(env, base);
    expr.pres.insert(0, construct);
    expr
}
