use super::{helpers, value};
use crate::{
    stage1::{
        self,
        structure::{
            construct::{Construct, ConstructBody},
            expression::Expression,
            statement::{Is, Replace, Statement},
        },
    },
    stage2::structure::{
        BuiltinOperation, BuiltinValue, Definitions, Environment, Item, Namespace, NamespaceId,
        Value, ValueId,
    },
};

pub fn vomit(
    definitions: &Definitions,
    env: &Environment,
    base: &NamespaceId,
    item: Item,
) -> Expression {
    let statements = build_statements(definitions, env);
    let expr = build_expr(statements, base, item, env);
    expr
}

fn build_statements(definitions: &Definitions, env: &Environment) -> Vec<Statement> {
    let mut statements = Vec::new();
    for (name, value) in definitions {
        let name = helpers::just_root_expression(helpers::identifier(name));
        let value = super::vomit(env, *value);
        let statement = Is { name, value };
        statements.push(Statement::Is(statement));
    }
    statements
}

fn build_expr(
    statements: Vec<Statement>,
    base: &crate::shared::Id<Option<Namespace>>,
    item: Item,
    env: &Environment,
) -> Expression {
    let construct = helpers::statements_construct("defining", statements);
    let base_item = Item {
        namespace: *base,
        value: item.value,
    };
    let mut expr = super::vomit(env, base_item);
    expr.others.push(construct);
    expr
}
