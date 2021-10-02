use crate::{
    stage1::structure::construct::Construct,
    stage2::{
        self,
        structure::{Environment, Item, Namespace, NamespaceId, Value, ValueId, Variable},
    },
};

pub fn ingest(env: &mut Environment, root: Construct, in_namespace: NamespaceId) -> Item {
    let typee = get_type_definition(root, env, in_namespace);
    let definition = define_variable(env, typee);
    create_variable_item(env, definition)
}

fn get_type_definition(root: Construct, env: &mut Environment, in_namespace: NamespaceId) -> Item {
    let typee = root
        .expect_single_expression("any")
        .expect("TODO: Nice error");
    let typee = stage2::ingest(env, typee.clone(), in_namespace);
    typee
}

fn define_variable(env: &mut Environment, typee: Item) -> ValueId {
    let definition = env.new_undefined_value();
    let variable = Variable {
        definition,
        original_type: typee.value,
    };
    let variable = env.variables.push(variable);
    let value = Value::Any { variable };
    env.define_value(definition, value);
    definition
}

fn create_variable_item(env: &mut Environment, definition: ValueId) -> Item {
    let namespace = env.insert_namespace(Namespace::Empty);
    let value = definition;
    Item { namespace, value }
}
