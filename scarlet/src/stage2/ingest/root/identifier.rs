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
    let the_name = root.expect_ident().unwrap().to_owned();
    let name = the_name.clone();
    let namespace = env.insert_namespace(Namespace::Identifier { name, in_namespace });
    let name = the_name;
    let value = env.insert_value(Value::Identifier { name, in_namespace });
    Item { namespace, value }
}
