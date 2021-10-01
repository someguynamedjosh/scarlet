use crate::{
    stage1::structure::construct::Construct,
    stage2::structure::{BuiltinValue, Environment, Item, Namespace, NamespaceId, Value},
};

pub fn ingest(env: &mut Environment, root: Construct, _in_namespace: NamespaceId) -> Item {
    let value = root.expect_text("u8").unwrap().parse().unwrap();
    let value = Value::BuiltinValue(BuiltinValue::U8(value));
    inserted_value(env, value)
}

fn inserted_value(env: &mut Environment, value: Value) -> Item {
    let namespace = env.insert_namespace(Namespace::Empty);
    let value = env.insert_value(value);
    Item { namespace, value }
}
