use crate::{
    stage1::structure::construct::Construct,
    stage2::{
        self,
        structure::{Environment, Item, Namespace, NamespaceId, Value, ValueId, Variant},
    },
};

pub fn ingest(env: &mut Environment, root: Construct, in_namespace: NamespaceId) -> Item {
    let typee = ingest_type(root, env, in_namespace);
    let definition = make_variant(env, typee);
    inserted_value(env, definition)
}

fn ingest_type(root: Construct, env: &mut Environment, in_namespace: NamespaceId) -> Item {
    let typee = root
        .expect_single_expression("variant")
        .expect("TODO: Nice error");
    let typee = stage2::ingest(env, typee.clone(), in_namespace);
    typee
}

fn make_variant(env: &mut Environment, typee: Item) -> ValueId {
    let definition = env.new_undefined_value();
    let variant = Variant {
        definition,
        original_type: typee.value,
    };
    let variant = env.variants.push(variant);
    let value = Value::Variant { variant };
    env.define_value(definition, value);
    definition
}

fn inserted_value(env: &mut Environment, definition: ValueId) -> Item {
    let namespace = env.insert_namespace(Namespace::Empty);
    let value = definition;
    Item { namespace, value }
}
