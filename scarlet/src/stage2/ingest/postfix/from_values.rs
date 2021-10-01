use crate::{
    stage1::structure::construct::Construct,
    stage2::{
        self,
        structure::{Environment, Item, Namespace, NamespaceId, Value, ValueId},
    },
};

pub fn ingest(
    env: &mut Environment,
    base: Item,
    post: Construct,
    in_namespace: NamespaceId,
) -> Item {
    let values = ingest_from_dependants(env, post, in_namespace);
    from_item(base, values, env)
}

fn from_item(base: Item, values: Vec<ValueId>, env: &mut Environment) -> Item {
    let value = Value::From {
        base: base.value,
        values,
    };
    let value = env.insert_value(value);
    let namespace = env.insert_namespace(Namespace::Empty);
    Item { namespace, value }
}

fn ingest_from_dependants(
    env: &mut Environment,
    post: Construct,
    in_namespace: NamespaceId,
) -> Vec<ValueId> {
    let items = post.expect_statements("FromValues").unwrap();
    let items = items
        .iter()
        .map(|i| i.expect_expression().expect("TODO: Nice error"));
    let items = items.map(|i| stage2::ingest(env, i.clone(), in_namespace));
    let values = items.map(|i| i.value).collect();
    values
}