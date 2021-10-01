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

pub fn ingest(env: &mut Environment, root: Construct, in_namespace: NamespaceId) -> Item {
    let typee = root
        .expect_single_expression("any")
        .expect("TODO: Nice error");
    let typee = stage2::ingest(env, typee.clone(), in_namespace);

    let definition = env.new_undefined_value();

    let variable = Variable {
        definition,
        original_type: typee.value,
    };
    let variable = env.variables.push(variable);

    let value = Value::Any { variable };
    env.define_value(definition, value);

    let namespace = env.insert_namespace(Namespace::Empty);
    let value = definition;
    Item { namespace, value }
}
