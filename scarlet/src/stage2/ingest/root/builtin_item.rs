use crate::{
    stage1::structure::{construct::Construct, expression::Expression},
    stage2::{
        self,
        structure::{
            BuiltinOperation, BuiltinValue, Environment, Item, Namespace, NamespaceId, Value,
            ValueId,
        },
    },
};

pub fn ingest(env: &mut Environment, root: Construct, in_namespace: NamespaceId) -> Item {
    let args = ingest_args(&root);
    let (name, args) = reduce_args(env, in_namespace, args);
    let value = ingest_builtin_value(&name, args);

    create_item(env, value)
}

fn ingest_args(root: &Construct) -> Vec<&Expression> {
    let args: Vec<_> = root
        .expect_statements("builtin_item")
        .unwrap()
        .iter()
        .map(|s| s.expect_expression().expect("TODO: Nice error"))
        .collect();
    args
}

fn reduce_args(
    env: &mut Environment,
    in_namespace: NamespaceId,
    mut args: Vec<&Expression>,
) -> (String, Vec<ValueId>) {
    if args.len() < 1 {
        todo!("nice error");
    }
    (
        args.remove(0)
            .expect_ident()
            .expect("TODO: Nice error")
            .to_owned(),
        args.into_iter()
            .map(|arg| stage2::ingest(env, arg.clone(), in_namespace).value)
            .collect(),
    )
}

fn ingest_builtin_value(name: &str, args: Vec<ValueId>) -> Value {
    let value = match name {
        "TYPE" => ingest_origin_type(args),
        "UnsignedInteger8" => ingest_u8_type(args),
        "cast" => ingest_cast(args),
        _ => todo!("Nice error, {} is not a recognized builtin item", name),
    };
    value
}

fn ingest_origin_type(args: Vec<ValueId>) -> Value {
    assert_eq!(args.len(), 0, "TODO: Nice error");
    Value::BuiltinValue(BuiltinValue::OriginType)
}

fn ingest_u8_type(args: Vec<ValueId>) -> Value {
    assert_eq!(args.len(), 0, "TODO: Nice error");
    Value::BuiltinValue(BuiltinValue::U8Type)
}

fn ingest_cast(args: Vec<ValueId>) -> Value {
    assert_eq!(args.len(), 4, "TODO: Nice error");
    Value::BuiltinOperation(BuiltinOperation::Cast {
        equality_proof: args[0],
        original_type: args[1],
        new_type: args[2],
        original_value: args[3],
    })
}

fn create_item(env: &mut Environment, value: Value) -> Item {
    let namespace = env.insert_namespace(Namespace::Empty);
    let value = env.insert_value(value);
    Item { namespace, value }
}
