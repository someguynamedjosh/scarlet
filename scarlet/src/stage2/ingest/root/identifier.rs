use crate::{
    stage1::structure::construct::Construct,
    stage2::structure::{Environment, Item, Namespace, NamespaceId, Value},
};

pub fn ingest(env: &mut Environment, root: Construct, in_namespace: NamespaceId) -> Item {
    let the_name = root.expect_ident().unwrap().to_owned();
    let name = the_name.clone();
    let namespace = env.insert_namespace(Namespace::Identifier { name, in_namespace });
    let name = the_name;
    let value = env.insert_value(Value::Identifier { name, in_namespace });
    Item { namespace, value }
}
