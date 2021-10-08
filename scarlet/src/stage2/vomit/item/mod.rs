use super::helpers;
use crate::{
    shared::OpaqueClass,
    stage1::structure::{construct::Construct, expression::Expression},
    stage2::{
        completely_vomit_item,
        structure::{BuiltinOperation, BuiltinValue, Environment, Item, ItemId},
    },
};

mod defining;
mod member;
mod substituting;

pub fn vomit(env: &Environment, item: ItemId) -> Expression {
    let item = &env.items[item];
    match &item.item {
        Item::BuiltinOperation(op) => vomit_operation(env, op),
        Item::BuiltinValue(val) => vomit_builtin_value(val),
        Item::Defining { base, definitions } => defining::vomit(env, definitions, *base),
        Item::From { base, value } => return vomit_from(env, *value, *base),
        Item::Identifier(name) => vomit_identifier(name),
        Item::Match { base, cases } => vomit_match(env, *base, cases),
        Item::Member { base, name } => member::vomit(env, *base, name),
        Item::Opaque { class, typee, .. } => vomit_opaque(env, *class, *typee),
        Item::Substituting {
            base,
            target,
            value,
        } => substituting::vomit(env, *target, *value, *base),
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

fn vomit_from(env: &Environment, value: ItemId, base: ItemId) -> Expression {
    let expressions = vec![vomit(env, value)];
    let mut result = vomit(env, base);
    result
        .pres
        .insert(0, helpers::expressions_construct("FromValues", expressions));
    result
}

fn vomit_match(env: &Environment, base: ItemId, cases: &[(ItemId, ItemId)]) -> Expression {
    let expressions = cases.iter().map(|c| vomit_case(env, c)).collect();
    let mut result = vomit(env, base);
    let match_construct = helpers::expressions_construct("matching", expressions);
    result.posts.push(match_construct);
    result
}

fn vomit_case(env: &Environment, (case, value): &(ItemId, ItemId)) -> Expression {
    let case = vomit(env, *case);
    let mut result = vomit(env, *value);
    result.pres.push(helpers::single_expr_construct("on", case));
    result
}

fn vomit_opaque(env: &Environment, class: OpaqueClass, typee: ItemId) -> Expression {
    let typee = vomit(env, typee);
    let label = match class {
        OpaqueClass::Variable => "any",
        OpaqueClass::Variant => "variant",
    };
    let construct = helpers::single_expr_construct(label, typee);
    helpers::just_root_expression(construct)
}
