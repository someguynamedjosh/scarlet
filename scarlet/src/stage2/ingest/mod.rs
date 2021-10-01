use super::structure::{
    BuiltinValue, Definitions, Environment, Item, Namespace, NamespaceId, Value,
};
use crate::{
    stage1::structure::{construct::Construct, expression::Expression, statement::Statement},
    stage2::structure::{BuiltinOperation, Replacements, Variable, Variant},
};

pub fn ingest(
    env: &mut Environment,
    mut expression: Expression,
    in_namespace: NamespaceId,
) -> Item {
    let result = if let Some(post) = expression.others.pop() {
        if post.label == "defining" {
            ingest_defining_postfix_construct(env, expression, post, in_namespace)
        } else {
            let base = ingest(env, expression, in_namespace);
            ingest_non_defining_postfix_construct(env, base, post, in_namespace)
        }
    } else {
        ingest_root_construct(env, expression.root, in_namespace)
    };
    assert!(env[result.namespace].is_some());
    assert!(env[result.value].is_some());
    result
}

fn ingest_defining_postfix_construct(
    env: &mut Environment,
    remainder: Expression,
    post: Construct,
    in_namespace: NamespaceId,
) -> Item {
    let this_ns = env.new_undefined_namespace();
    let base = ingest(env, remainder, this_ns);
    let mut definitions = Definitions::new();
    for statement in post.expect_statements("defining").expect("TODO error") {
        match statement {
            Statement::Is(is) => {
                let name = is.name.expect_ident().expect("TODO error").to_owned();
                let item = ingest(env, is.value.clone(), this_ns);
                definitions.insert_no_replace(name, item);
            }
            _ => todo!("nice error"),
        }
    }
    env.define_namespace(
        this_ns,
        Namespace::Defining {
            base: base.namespace,
            definitions,
            parent: in_namespace,
        },
    );
    Item {
        namespace: this_ns,
        value: base.value,
    }
}

fn ingest_non_defining_postfix_construct(
    env: &mut Environment,
    base: Item,
    post: Construct,
    in_namespace: NamespaceId,
) -> Item {
    match &post.label[..] {
        "defining" => unreachable!(),
        "FromValues" => {
            let items = post.expect_statements("FromValues").unwrap();
            let items = items
                .iter()
                .map(|i| i.expect_expression().expect("TODO: Nice error"));
            let items = items.map(|i| ingest(env, i.clone(), in_namespace));
            let values = items.map(|i| i.value).collect();
            let value = Value::From {
                base: base.value,
                values,
            };
            let value = env.insert_value(value);
            let namespace = env.insert_namespace(Namespace::Empty);
            Item { namespace, value }
        }
        "member" => {
            let the_name = post
                .expect_single_expression("member")
                .expect("TODO: nice error")
                .expect_ident()
                .unwrap()
                .to_owned();
            let previous_value = base.value;
            let base = base.namespace;
            let name = the_name.clone();
            let namespace = env.insert_namespace(Namespace::Member { base, name });
            let name = the_name;
            let value = env.insert_value(Value::Member {
                base,
                name,
                previous_value,
            });
            Item { namespace, value }
        }
        "replacing" => {
            let mut replacements = Replacements::new();
            for statement in post.expect_statements("replacing").unwrap() {
                match statement {
                    Statement::Replace(replace) => {
                        let target = ingest(env, replace.target.clone(), in_namespace);
                        let value = ingest(env, replace.value.clone(), in_namespace);
                        replacements.push((target.value, value.value));
                    }
                    Statement::Expression(..) => todo!(),
                    _ => todo!("nice error"),
                }
            }
            let namespace = Namespace::Replacing {
                base: base.namespace,
                replacements: replacements.clone(),
            };
            let namespace = env.insert_namespace(namespace);
            let value = Value::Replacing {
                base: base.value,
                replacements,
            };
            let value = env.insert_value(value);
            Item { namespace, value }
        }
        "type_is" => todo!(),
        _ => todo!("nice error"),
    }
}

fn ingest_root_construct(
    env: &mut Environment,
    root: Construct,
    in_namespace: NamespaceId,
) -> Item {
    match &root.label[..] {
        "any" => {
            let typee = root
                .expect_single_expression("any")
                .expect("TODO: Nice error");
            let typee = ingest(env, typee.clone(), in_namespace);

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
        "builtin_item" => {
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
                .map(|arg| ingest(env, arg.clone(), in_namespace).value)
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
        "identifier" => {
            let the_name = root.expect_ident().unwrap().to_owned();
            let name = the_name.clone();
            let namespace = env.insert_namespace(Namespace::Identifier { name, in_namespace });
            let name = the_name;
            let value = env.insert_value(Value::Identifier { name, in_namespace });
            Item { namespace, value }
        }
        "u8" => {
            let namespace = env.insert_namespace(Namespace::Empty);
            let value = root.expect_text("u8").unwrap().parse().unwrap();
            let value = Value::BuiltinValue(BuiltinValue::U8(value));
            let value = env.insert_value(value);
            Item { namespace, value }
        }
        "variant" => {
            let typee = root
                .expect_single_expression("variant")
                .expect("TODO: Nice error");
            let typee = ingest(env, typee.clone(), in_namespace);

            let definition = env.new_undefined_value();

            let variant = Variant {
                definition,
                original_type: typee.value,
            };
            let variant = env.variants.push(variant);

            let value = Value::Variant { variant };
            env.define_value(definition, value);

            let namespace = env.insert_namespace(Namespace::Empty);
            let value = definition;
            Item { namespace, value }
        }
        _ => todo!("Nice error"),
    }
}
