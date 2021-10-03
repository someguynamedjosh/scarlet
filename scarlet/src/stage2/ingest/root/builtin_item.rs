use crate::{
    stage1::structure::{construct::Construct, expression::Expression},
    stage2::{
        self,
        structure::{BuiltinOperation, BuiltinValue, Item},
    },
};

pub fn ingest(root: Construct) -> Item {
    let args = ingest_args(&root);
    let (name, args) = reduce_args(args);
    ingest_builtin_value(&name, args)
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

fn reduce_args(mut args: Vec<&Expression>) -> (String, Vec<Item>) {
    if args.len() < 1 {
        todo!("nice error");
    }
    (
        args.remove(0)
            .expect_ident()
            .expect("TODO: Nice error")
            .to_owned(),
        args.into_iter()
            .map(|arg| stage2::ingest(arg.clone()))
            .collect(),
    )
}

fn ingest_builtin_value(name: &str, args: Vec<Item>) -> Item {
    let value = match name {
        "TYPE" => ingest_origin_type(args),
        "UnsignedInteger8" => ingest_u8_type(args),
        "cast" => ingest_cast(args),
        _ => todo!("Nice error, {} is not a recognized builtin item", name),
    };
    value
}

fn ingest_origin_type(args: Vec<Item>) -> Item {
    assert_eq!(args.len(), 0, "TODO: Nice error");
    Item::BuiltinValue(BuiltinValue::OriginType)
}

fn ingest_u8_type(args: Vec<Item>) -> Item {
    assert_eq!(args.len(), 0, "TODO: Nice error");
    Item::BuiltinValue(BuiltinValue::U8Type)
}

fn ingest_cast(args: Vec<Item>) -> Item {
    assert_eq!(args.len(), 4, "TODO: Nice error");
    Item::BuiltinOperation(Box::new(BuiltinOperation::Cast {
        equality_proof: args[0].clone(),
        original_type: args[1].clone(),
        new_type: args[2].clone(),
        original_value: args[3].clone(),
    }))
}
