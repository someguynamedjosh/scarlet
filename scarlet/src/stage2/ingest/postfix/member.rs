use crate::{
    stage1::structure::construct::Construct,
    stage2::structure::{Environment, Item, Namespace, NamespaceId, Value, ValueId},
};

pub fn ingest(
    env: &mut Environment,
    base: Item,
    post: Construct,
    _in_namespace: NamespaceId,
) -> Item {
    let the_name = ingest_ident_name(post);
    let namespace = member_namespace(env, base.namespace, the_name.clone());
    let value = member_value(env, base.namespace, the_name, base.value);
    Item { namespace, value }
}

fn member_value(
    env: &mut Environment,
    base: NamespaceId,
    name: String,
    previous_value: ValueId,
) -> ValueId {
    env.insert_value(Value::Member {
        base,
        name,
        previous_value,
    })
}

fn member_namespace(env: &mut Environment, base: NamespaceId, name: String) -> NamespaceId {
    env.insert_namespace(Namespace::Member { base, name })
}

fn ingest_ident_name(post: Construct) -> String {
    let the_name = post
        .expect_single_expression("member")
        .expect("TODO: nice error")
        .expect_ident()
        .unwrap()
        .to_owned();
    the_name
}
