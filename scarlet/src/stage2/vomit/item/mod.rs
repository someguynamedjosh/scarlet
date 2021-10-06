use super::helpers;
use crate::{
    stage1::structure::{construct::Construct, expression::Expression},
    stage2::structure::{BuiltinOperation, BuiltinValue, Environment, Item, ItemId},
};

mod defining;
mod member;
mod substituting;

pub fn vomit(env: &Environment, item: ItemId) -> Expression {
    let item = &env.items[item];
    match item {
        Item::Any { typee, .. } => vomit_any(env, *typee),
        Item::BuiltinOperation(op) => vomit_operation(env, op),
        Item::BuiltinValue(val) => vomit_builtin_value(val),
        Item::Defining { base, definitions } => defining::vomit(env, definitions, *base),
        Item::From { base, values } => return vomit_from(env, values, *base),
        Item::Identifier(name) => vomit_identifier(name),
        Item::Member { base, name } => member::vomit(env, *base, name),
        Item::Substituting {
            base,
            substitutions,
        } => substituting::vomit(env, substitutions, *base),
        Item::Variant { typee, .. } => vomit_variant(env, *typee),
    }
}

fn vomit_any(env: &Environment, typee: ItemId) -> Expression {
    let typee = vomit(env, typee);
    let construct = helpers::single_expr_construct("any", typee);
    helpers::just_root_expression(construct)
}

fn vomit_identifier(name: &String) -> Expression {
    helpers::just_root_expression(helpers::identifier(name))
}

fn vomit_operation(env: &Environment, op: &BuiltinOperation<ItemId>) -> Expression {
    let name = match op {
        BuiltinOperation::Cast { .. } => "cast",
    };
    let mut exprs = vec![helpers::just_root_expression(helpers::identifier(name))];
    for arg in op.inputs() {
        exprs.push(vomit(env, *arg));
    }
    let construct = helpers::expressions_construct("builtin_item", exprs);
    helpers::just_root_expression(construct)
}

fn vomit_builtin_value(val: &BuiltinValue) -> Expression {
    let construct = match val {
        BuiltinValue::OriginType => helpers::simple_builtin_item("TYPE"),
        BuiltinValue::U8(value) => helpers::text_construct("u8", format!("{}", value)),
        BuiltinValue::U8Type => helpers::simple_builtin_item("UnsignedInteger8"),
    };
    helpers::just_root_expression(construct)
}

fn vomit_from(env: &Environment, values: &Vec<ItemId>, base: ItemId) -> Expression {
    let values = values.iter().map(|i| vomit(env, *i));
    let mut result = vomit(env, base);
    result.pres.insert(
        0,
        helpers::expressions_construct("FromValues", values.collect()),
    );
    result
}

fn vomit_variant(env: &Environment, typee: ItemId) -> Expression {
    let typee = vomit(env, typee);
    let construct = helpers::single_expr_construct("variant", typee);
    helpers::just_root_expression(construct)
}
