use crate::{
    stage1::structure::construct::Construct,
    stage2::structure::{Environment, Item, Namespace, NamespaceId, Value},
};

pub fn ingest(env: &mut Environment, root: Construct, in_namespace: NamespaceId) -> Item {
    let name = root.expect_ident().unwrap().to_owned();
    let namespace = build_namespace(env, name.clone(), in_namespace);
    let value = build_value(env, name, in_namespace);
    Item { namespace, value }
}

fn build_namespace(
    env: &mut Environment,
    name: String,
    in_namespace: crate::shared::Id<Option<Namespace>>,
) -> crate::shared::Id<Option<Namespace>> {
    let namespace = env.insert_namespace(Namespace::Identifier { name, in_namespace });
    namespace
}

fn build_value(
    env: &mut Environment,
    name: String,
    in_namespace: crate::shared::Id<Option<Namespace>>,
) -> crate::shared::Id<Option<Value>> {
    let value = env.insert_value(Value::Identifier { name, in_namespace });
    value
}
