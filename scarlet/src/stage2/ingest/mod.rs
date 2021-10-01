use super::structure::{
    BuiltinValue, Definitions, Environment, Item, Namespace, NamespaceId, Value,
};
use crate::{
    stage1::structure::{construct::Construct, expression::Expression, statement::Statement},
    stage2::structure::{BuiltinOperation, Replacements, Variable, Variant},
};

mod postfix;
mod root;

pub fn ingest(
    env: &mut Environment,
    mut expression: Expression,
    in_namespace: NamespaceId,
) -> Item {
    let result = if let Some(post) = expression.others.pop() {
        postfix::ingest(env, expression, post, in_namespace)
    } else {
        root::ingest(env, expression.root, in_namespace)
    };
    assert!(env[result.namespace].is_some());
    assert!(env[result.value].is_some());
    result
}
