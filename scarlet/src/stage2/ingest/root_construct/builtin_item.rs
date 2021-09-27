use crate::{
    shared::{BuiltinOperation, IntegerMathOperation, Item, PrimitiveType, PrimitiveValue},
    stage1::structure::construct::Construct,
    stage2::{
        ingest::{context::Context, expression::ingest_expression},
        structure::UnresolvedItem,
    },
};

pub fn ingest_builtin_item(ctx: &mut Context, root: Construct) -> Result<UnresolvedItem, String> {
    let statements = root.expect_statements("builtin_item")?;
    let name = statements[0].expect_expression()?.expect_ident()?;

    let mut args = Vec::new();
    for statement in &statements[1..] {
        let expr = statement.expect_expression()?;
        args.push(ingest_expression(ctx, expr.clone(), vec![])?);
    }

    let item = match name {
        "reinterpret" => {
            if args.len() != 4 {
                todo!("nice error, wrong number of arguments");
            }
            Item::BuiltinOperation(BuiltinOperation::Reinterpret {
                proof_equal: args[0],
                original_type: args[1],
                new_type: args[2],
                original: args[3],
            })
        }
        "TYPE" => {
            if args.len() != 0 {
                todo!("nice error, wrong number of arguments");
            }
            Item::GodType
        }
        "Integer32" => {
            if args.len() != 0 {
                todo!("nice error, wrong number of arguments");
            }
            Item::PrimitiveType(PrimitiveType::I32)
        }
        "i32_sum" => {
            if args.len() != 2 {
                todo!("nice error, wrong number of arguments");
            }
            Item::BuiltinOperation(BuiltinOperation::I32Math(IntegerMathOperation::Sum(
                args[0], args[1],
            )))
        }
        "i32_difference" => {
            if args.len() != 2 {
                todo!("nice error, wrong number of arguments");
            }
            Item::BuiltinOperation(BuiltinOperation::I32Math(IntegerMathOperation::Difference(
                args[0], args[1],
            )))
        }
        "Boolean" => {
            if args.len() != 0 {
                todo!("nice error, wrong number of arguments");
            }
            Item::PrimitiveType(PrimitiveType::Bool)
        }
        "true" => {
            if args.len() != 0 {
                todo!("nice error, wrong number of arguments");
            }
            Item::PrimitiveValue(PrimitiveValue::Bool(true))
        }
        "false" => {
            if args.len() != 0 {
                todo!("nice error, wrong number of arguments");
            }
            Item::PrimitiveValue(PrimitiveValue::Bool(false))
        }
        "are_same_variant" => {
            if args.len() != 2 {
                todo!("nice error, wrong number of arguments");
            }
            Item::BuiltinOperation(BuiltinOperation::AreSameVariant {
                base: args[0],
                other: args[1],
            })
        }
        other => todo!("Unrecognized builtin_item: {}", other),
    };
    Ok(UnresolvedItem::Just(item))
}
