use super::{helpers, value};
use crate::{
    stage1::structure::{
        expression::Expression,
        statement::{Replace, Statement},
    },
    stage2::structure::{Environment, Item, NamespaceId, Value, ValueId},
};

pub fn vomit(
    value: &Value,
    replacements: &Vec<(ValueId, ValueId)>,
    base: &NamespaceId,
    env: &Environment,
) -> Expression {
    if let Value::Replacing {
        base: vbase,
        replacements: vreplacements,
    } = value
    {
        vomit_impl(replacements, vreplacements, vbase, base, env)
    } else {
        unreachable!("ICE")
    }
}

fn vomit_impl(
    replacements: &Vec<(ValueId, ValueId)>,
    vreplacements: &Vec<(ValueId, ValueId)>,
    vbase: &ValueId,
    base: &NamespaceId,
    env: &Environment,
) -> Expression {
    debug_assert_eq!(replacements, vreplacements);
    let base = Item {
        value: *vbase,
        namespace: *base,
    };
    let statements = build_statements(replacements, env);
    build_replacing_expr(statements, env, base)
}

fn build_statements(
    replacements: &Vec<(
        crate::shared::Id<Option<Value>>,
        crate::shared::Id<Option<Value>>,
    )>,
    env: &Environment,
) -> Vec<Statement> {
    let mut statements = Vec::new();
    for (target, value) in replacements {
        statements.push(build_statement(env, target, value));
    }
    statements
}

fn build_statement(
    env: &Environment,
    target: &crate::shared::Id<Option<Value>>,
    value: &crate::shared::Id<Option<Value>>,
) -> Statement {
    let target = value::vomit_value(env, *target);
    let value = value::vomit_value(env, *value);
    let statement = Replace { target, value };
    let statement = Statement::Replace(statement);
    statement
}

fn build_replacing_expr(statements: Vec<Statement>, env: &Environment, base: Item) -> Expression {
    let replacing = helpers::statements_construct("replacing", statements);
    let mut base = super::vomit(env, base);
    base.others.push(replacing);
    base
}