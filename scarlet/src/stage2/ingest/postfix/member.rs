use crate::{
    stage1::structure::construct::Construct,
    stage2::structure::{Environment, Item, Namespace, NamespaceId, Value},
};

pub fn ingest(
    env: &mut Environment,
    base: Item,
    post: Construct,
    _in_namespace: NamespaceId,
) -> Item {
    let the_name = post
        .expect_single_expression("member")
        .expect("TODO: nice error")
        .expect_ident()
        .unwrap()
        .to_owned();
    let previous_value = base.value;
    let base = base.namespace;
    let name = the_name.clone();
    let namespace = env.insert_namespace(Namespace::Member { base, name });
    let name = the_name;
    let value = env.insert_value(Value::Member {
        base,
        name,
        previous_value,
    });
    Item { namespace, value }
}
