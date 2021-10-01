use super::structure::{
    BuiltinValue, Definitions, Environment, Item, Namespace, NamespaceId, Value,
};
use crate::stage1::structure::{
    construct::Construct, expression::Expression, statement::Statement,
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
        "FromItems" => todo!(),
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
        "replacing" => todo!(),
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
        "any" => todo!(),
        "builtin_item" => todo!(),
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
        "variant" => todo!(),
        _ => todo!("Nice error"),
    }
}
