use super::structure::{Definitions, Environment, ItemId, ScopeId, Value};
use crate::{
    stage1::structure::{construct::Construct, expression::Expression, statement::Statement},
    stage2::structure::{BuiltinValue, Scope},
};

pub fn ingest(env: &mut Environment, mut expr: Expression, into: ItemId) -> Result<(), String> {
    if let Some(post) = expr.others.pop() {
        ingest_postfix_construct(env, expr, post, into)?;
    } else {
        ingest_root_construct(env, expr.root, into)?;
    }
    assert!(env[into].value.is_some());
    Ok(())
}

fn ingest_root_construct(
    env: &mut Environment,
    root: Construct,
    into: ItemId,
) -> Result<(), String> {
    match &root.label[..] {
        "any" => todo!(),
        "builtin_item" => todo!(),
        "identifier" => todo!(),
        "u8" => {
            let value = root.expect_text("u8").unwrap();
            let value: u8 = value.parse().unwrap();
            let value = Value::BuiltinValue {
                value: BuiltinValue::U8(value),
            };
            env.define_item_value(into, value);
            Ok(())
        }
        "variant" => todo!(),
        other => todo!("nice error, {} is not a valid root construct.", other),
    }
}

fn ingest_postfix_construct(
    env: &mut Environment,
    remainder: Expression,
    post: Construct,
    into: ItemId,
) -> Result<(), String> {
    if post.label == "defining" {
        ingest_defining_construct(env, remainder, post, into)
    } else {
        match &post.label[..] {
            "defining" => unreachable!(),
            "FromItems" => todo!(),
            "member" => todo!(),
            "replacing" => todo!(),
            "type_is" => todo!(),
            other => todo!("nice error, {} is not a valid postfix construct.", other),
        }
    }
}

fn ingest_defining_construct(
    env: &mut Environment,
    remainder: Expression,
    post: Construct,
    self_id: ItemId,
) -> Result<(), String> {
    let self_scope = env.scopes.push(Scope {
        definition: Some(self_id),
    });
    let base_id = env.new_undefined_item(self_scope);
    ingest(env, remainder, base_id)?;

    let mut definitions = Definitions::new();
    for statement in post.expect_statements("defining")? {
        let id = env.new_undefined_item(self_scope);
        match statement {
            Statement::Is(s) => {
                ingest(env, s.value.clone(), id)?;
                let key = s.name.expect_ident()?;
                if definitions.contains_key(key) {
                    todo!("Nice error, multiple definitions with name {}", key);
                }
                definitions.insert_no_replace((key.to_owned(), id));
            }
            _ => todo!(),
        }
    }

    let self_value = Value::Defining {
        base: base_id,
        definitions,
        this_scope: self_scope,
    };
    env.define_item_value(self_id, self_value);
    Ok(())
}
