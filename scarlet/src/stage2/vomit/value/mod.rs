use super::helpers;
use crate::{
    stage1::structure::{construct::Construct, expression::Expression},
    stage2::structure::{BuiltinOperation, BuiltinValue, Environment, Value, ValueId, VariableId},
};

pub fn vomit_value(env: &Environment, value: ValueId) -> Expression {
    let value = env[value].as_ref().expect("TODO: Nice error");
    vomit_value_impl(env, value)
}

pub fn vomit_value_impl(env: &Environment, value: &Value) -> Expression {
    let construct = match value {
        Value::Any { variable } => vomit_any(env, variable),
        Value::BuiltinOperation(op) => vomit_operation(op, env),
        Value::BuiltinValue(val) => vomit_builtin_value(val),
        Value::From { base, values } => return vomit_from(values, env, base),
        Value::Identifier { name, .. } => helpers::identifier(name),
        Value::Member { .. } => unreachable!(),
        Value::Replacing { .. } => todo!(),
        Value::Variant { .. } => todo!(),
    };
    helpers::just_root_expression(construct)
}

fn vomit_any(env: &Environment, variable: &VariableId) -> Construct {
    let typee = env[*variable].original_type;
    let typee = vomit_value(env, typee);
    helpers::single_expr_construct("any", typee)
}

fn vomit_operation(op: &BuiltinOperation, env: &Environment) -> Construct {
    let name = match op {
        BuiltinOperation::Cast { .. } => "cast",
    };
    let mut exprs = vec![helpers::just_root_expression(helpers::identifier(name))];
    for arg in op.inputs() {
        exprs.push(vomit_value(env, arg));
    }
    helpers::expressions_construct("builtin_item", exprs)
}

fn vomit_builtin_value(val: &BuiltinValue) -> Construct {
    match val {
        BuiltinValue::OriginType => helpers::simple_builtin_item("TYPE"),
        BuiltinValue::U8(value) => helpers::text_construct("u8", format!("{}", value)),
        BuiltinValue::U8Type => helpers::simple_builtin_item("UnsignedInteger8"),
    }
}

fn vomit_from(values: &Vec<ValueId>, env: &Environment, base: &ValueId) -> Expression {
    let values = values.iter().map(|v| vomit_value(env, *v));
    let mut result = vomit_value(env, *base);
    result.others.push(helpers::expressions_construct(
        "FromValues",
        values.collect(),
    ));
    result
}
