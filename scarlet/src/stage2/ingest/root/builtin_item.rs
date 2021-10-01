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
    let args = root.expect_statements("builtin_item").unwrap();
    let mut args: Vec<_> = args
        .iter()
        .map(|s| s.expect_expression().expect("TODO: Nice error"))
        .collect();
    if args.len() < 1 {
        todo!("nice error");
    }

    let name = args.remove(0).expect_ident().expect("TODO: Nice error");
    let args: Vec<_> = args
        .into_iter()
        .map(|arg| stage2::ingest(env, arg.clone(), in_namespace).value)
        .collect();
    let value = match name {
        "TYPE" => {
            assert_eq!(args.len(), 0, "TODO: Nice error");
            Value::BuiltinValue(BuiltinValue::OriginType)
        }
        "UnsignedInteger8" => {
            assert_eq!(args.len(), 0, "TODO: Nice error");
            Value::BuiltinValue(BuiltinValue::U8Type)
        }
        "cast" => {
            assert_eq!(args.len(), 4, "TODO: Nice error");
            Value::BuiltinOperation(BuiltinOperation::Cast {
                equality_proof: args[0],
                original_type: args[1],
                new_type: args[2],
                original_value: args[3],
            })
        }
        _ => todo!("Nice error, {} is not a recognized builtin item", name),
    };

    let namespace = env.insert_namespace(Namespace::Empty);
    let value = env.insert_value(value);
    Item { namespace, value }
}
