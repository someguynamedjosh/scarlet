use crate::{
    stage1::structure::{construct::Construct, expression::Expression, statement::Statement},
    stage2::{
        self,
        structure::{
            BuiltinOperation, BuiltinValue, Definitions, Environment, Item, Namespace, NamespaceId,
            Replacements, Value, Variable, Variant,
        },
    },
};

mod defining;
mod from_values;
mod member;
mod replacing;

pub fn ingest(
    env: &mut Environment,
    remainder: Expression,
    post: Construct,
    in_namespace: NamespaceId,
) -> Item {
    if post.label == "defining" {
        defining::ingest(env, remainder, post, in_namespace)
    } else {
        let base = stage2::ingest(env, remainder, in_namespace);
        ingest_non_defining(env, base, post, in_namespace)
    }
}

fn ingest_non_defining(
    env: &mut Environment,
    base: Item,
    post: Construct,
    in_namespace: NamespaceId,
) -> Item {
    match &post.label[..] {
        "defining" => unreachable!(),
        "FromValues" => from_values::ingest(env, base, post, in_namespace),
        "member" => member::ingest(env, base, post, in_namespace),
        "replacing" => replacing::ingest(env, base, post, in_namespace),
        "type_is" => todo!(),
        _ => todo!("nice error"),
    }
}
