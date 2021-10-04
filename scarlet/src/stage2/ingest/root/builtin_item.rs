use crate::{
    stage1::structure::{construct::Construct, expression::Expression},
    stage2::{
        self,
        structure::{BuiltinOperation, BuiltinValue, Environment, Item, ItemId},
    },
};

pub fn ingest(env: &mut Environment, root: Construct) -> ItemId {
    let args = ingest_args(&root);
    let (name, args) = reduce_args(env, args);
    env.push_item(ingest_builtin_value(&name, args))
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

fn reduce_args(env: &mut Environment, mut args: Vec<&Expression>) -> (String, Vec<ItemId>) {
    if args.len() < 1 {
        todo!("nice error");
    }
    (
        args.remove(0)
            .expect_ident()
            .expect("TODO: Nice error")
            .to_owned(),
        args.into_iter()
            .map(|arg| stage2::ingest_expression(env, arg.clone()))
            .collect(),
    )
}

fn ingest_builtin_value(name: &str, args: Vec<ItemId>) -> Item {
    let value = match name {
        "TYPE" => ingest_origin_type(args),
        "UnsignedInteger8" => ingest_u8_type(args),
        "cast" => ingest_cast(args),
        _ => todo!("Nice error, {} is not a recognized builtin item", name),
    };
    value
}

fn ingest_origin_type(args: Vec<ItemId>) -> Item {
    assert_eq!(args.len(), 0, "TODO: Nice error");
    Item::BuiltinValue(BuiltinValue::OriginType)
}

fn ingest_u8_type(args: Vec<ItemId>) -> Item {
    assert_eq!(args.len(), 0, "TODO: Nice error");
    Item::BuiltinValue(BuiltinValue::U8Type)
}

fn ingest_cast(args: Vec<ItemId>) -> Item {
    assert_eq!(args.len(), 4, "TODO: Nice error");
    Item::BuiltinOperation(BuiltinOperation::Cast {
        equality_proof: args[0],
        original_type: args[1],
        new_type: args[2],
        original_value: args[3],
    })
}
