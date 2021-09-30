use super::structure::{Definitions, Environment, ItemId, ScopeId, Value};
use crate::{
    stage1::structure::{construct::Construct, expression::Expression, statement::Statement},
    stage2::structure::{BuiltinValue, ItemReplacements, Scope, Variable, Variant},
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
    let defined_in = env[into].defined_in;
    match &root.label[..] {
        "any" => {
            let typee = root.expect_single_expression("any")?;
            let type_id = env.new_undefined_item(defined_in);
            ingest(env, typee.clone(), type_id)?;
            let variable = env.variables.push(Variable {
                definition: into,
                original_type: type_id,
            });
            let value = Value::Any { variable };
            env.define_item_value(into, value);
            Ok(())
        }
        "builtin_item" => {
            let statements = root.expect_statements("builtin_item")?;
            let mut args = Vec::new();
            for statement in statements {
                if let Statement::Expression(expr) = statement {
                    args.push(expr);
                } else {
                    todo!("Nice error, expected expression.");
                }
            }
            if args.len() == 0 {
                todo!("Nice error, require at least one expression.");
            }
            let name = args.remove(0).expect_ident()?;
            let args = args;
            let value = match name {
                "UnsignedInteger8" => {
                    assert_eq!(args.len(), 0, "TOODO: Nice error, wrong number of args");
                    Value::BuiltinValue {
                        value: BuiltinValue::U8Type,
                    }
                }
                other => todo!(
                    "Nice error, {} is not a recognized builtin item name",
                    other
                ),
            };
            env.define_item_value(into, value);
            Ok(())
        }
        "identifier" => {
            let name = root.expect_text("identifier")?.to_owned();
            let value = Value::Identifier { name };
            env.define_item_value(into, value);
            Ok(())
        }
        "u8" => {
            let value = root.expect_text("u8")?;
            let value: u8 = value.parse().unwrap();
            let value = Value::BuiltinValue {
                value: BuiltinValue::U8(value),
            };
            env.define_item_value(into, value);
            Ok(())
        }
        "variant" => {
            let typee = root.expect_single_expression("variant")?;
            let type_id = env.new_undefined_item(defined_in);
            ingest(env, typee.clone(), type_id)?;
            let variant = env.variants.push(Variant {
                definition: into,
                original_type: type_id,
            });
            let value = Value::Variant { variant };
            env.define_item_value(into, value);
            Ok(())
        }
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
        let defined_in = env[into].defined_in;
        let base = env.new_undefined_item(defined_in);
        ingest(env, remainder, base)?;
        ingest_non_defining_postfix_construct(env, base, post, into)?;
        Ok(())
    }
}

fn ingest_defining_construct(
    env: &mut Environment,
    remainder: Expression,
    post: Construct,
    self_id: ItemId,
) -> Result<(), String> {
    let self_scope = env.scopes.push(Scope {
        definition: self_id,
    });
    let base_id = env.new_undefined_item(Some(self_scope));
    ingest(env, remainder, base_id)?;

    let mut definitions = Definitions::new();
    for statement in post.expect_statements("defining")? {
        let id = env.new_undefined_item(Some(self_scope));
        match statement {
            Statement::Is(s) => {
                ingest(env, s.value.clone(), id)?;
                let key = s.name.expect_ident()?.to_owned();
                if definitions.contains_key(&key) {
                    todo!("Nice error, multiple definitions with name {}", key);
                }
                definitions.insert_no_replace(key, id);
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

fn ingest_non_defining_postfix_construct(
    env: &mut Environment,
    base: ItemId,
    post: Construct,
    into: ItemId,
) -> Result<(), String> {
    let defined_in = env[into].defined_in;
    match &post.label[..] {
        "defining" => unreachable!(),
        "FromItems" => {
            let mut items = Vec::new();
            for statement in post.expect_statements("defining")? {
                let id = env.new_undefined_item(defined_in);
                match statement {
                    Statement::Expression(s) => {
                        ingest(env, s.clone(), id)?;
                    }
                    _ => todo!(),
                }
                items.push(id);
            }

            let self_value = Value::FromItems { base, items };
            env.define_item_value(into, self_value);
            Ok(())
        }
        "member" => {
            let name = post.expect_single_expression("member").unwrap();
            if name.others.len() > 0 {
                todo!("Member should only be a single identifier");
            }
            let name = name
                .root
                .expect_ident()
                .expect("TODO: nice error")
                .to_owned();
            let value = Value::Member { base, name };
            env.define_item_value(into, value);
            Ok(())
        }
        "replacing" => {
            let mut replacements = ItemReplacements::new();
            for statement in post.expect_statements("replacing")? {
                let id = env.new_undefined_item(defined_in);
                match statement {
                    Statement::Replace(s) => {
                        ingest(env, s.value.clone(), id)?;
                        let target_id = env.new_undefined_item(defined_in);
                        ingest(env, s.target.clone(), target_id)?;
                        if replacements.contains_key(&target_id) {
                            todo!("Nice error, multiple replacements of {:?}", target_id);
                        }
                        replacements.insert_no_replace(target_id, id);
                    }
                    _ => todo!(),
                }
            }

            let self_value = Value::ReplacingItems { base, replacements };
            env.define_item_value(into, self_value);
            Ok(())
        }
        "type_is" => todo!(),
        other => todo!("nice error, {} is not a valid postfix construct.", other),
    }
}
