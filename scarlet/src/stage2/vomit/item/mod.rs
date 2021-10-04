use super::helpers;
use crate::{
    stage1::structure::{construct::Construct, expression::Expression},
    stage2::structure::{BuiltinOperation, BuiltinValue, Item},
};

mod defining;
mod member;
mod replacing;

pub fn vomit(item: &Item) -> Expression {
    match item {
        Item::Any { typee, .. } => vomit_any(typee),
        Item::BuiltinOperation(op) => vomit_operation(op),
        Item::BuiltinValue(val) => vomit_builtin_value(val),
        Item::Defining { base, definitions } => defining::vomit(definitions, base),
        Item::From { base, values } => return vomit_from(values, base),
        Item::Identifier(name) => vomit_identifier(name),
        Item::Member { base, name } => member::vomit(base, name),
        Item::Replacing { base, replacements } => replacing::vomit(replacements, base),
        Item::Variant { typee, .. } => vomit_variant(typee),
    }
}

fn vomit_any(typee: &Item) -> Expression {
    let typee = vomit(typee);
    let construct = helpers::single_expr_construct("any", typee);
    helpers::just_root_expression(construct)
}

fn vomit_identifier(name: &String) -> Expression {
    helpers::just_root_expression(helpers::identifier(name))
}

fn vomit_operation(op: &BuiltinOperation<Item>) -> Expression {
    let name = match op {
        BuiltinOperation::Cast { .. } => "cast",
    };
    let mut exprs = vec![helpers::just_root_expression(helpers::identifier(name))];
    for arg in op.inputs() {
        exprs.push(vomit(arg));
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

fn vomit_from(values: &Vec<Item>, base: &Item) -> Expression {
    let values = values.iter().map(vomit);
    let mut result = vomit(base);
    result.others.push(helpers::expressions_construct(
        "FromValues",
        values.collect(),
    ));
    result
}

fn vomit_variant(typee: &Item) -> Expression {
    let typee = vomit(typee);
    let construct = helpers::single_expr_construct("variant", typee);
    helpers::just_root_expression(construct)
}
