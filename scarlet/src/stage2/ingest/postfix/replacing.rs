use crate::{
    stage1::structure::{construct::Construct, statement::Statement},
    stage2::{
        self,
        structure::{Environment, Item, Namespace, NamespaceId, Replacements, Value, ValueId},
    },
};

pub fn ingest(
    env: &mut Environment,
    base: Item,
    post: Construct,
    in_namespace: NamespaceId,
) -> Item {
    let replacements = ingest_replacements(post, env, in_namespace);
    create_replacement_item(env, base, replacements)
}

fn ingest_replacements(
    post: Construct,
    env: &mut Environment,
    in_namespace: crate::shared::Id<Option<Namespace>>,
) -> Vec<(ValueId, ValueId)> {
    let mut replacements = Replacements::new();
    for statement in post.expect_statements("replacing").unwrap() {
        ingest_replacement(statement, env, in_namespace, &mut replacements);
    }
    replacements
}

fn ingest_replacement(
    statement: &Statement,
    env: &mut Environment,
    in_namespace: crate::shared::Id<Option<Namespace>>,
    replacements: &mut Vec<(ValueId, ValueId)>,
) {
    match statement {
        Statement::Replace(replace) => {
            let target = stage2::ingest(env, replace.target.clone(), in_namespace);
            let value = stage2::ingest(env, replace.value.clone(), in_namespace);
            replacements.push((target.value, value.value));
        }
        Statement::Expression(..) => todo!(),
        _ => todo!("nice error"),
    }
}

fn create_replacement_item(env: &mut Environment, base: Item, replacements: Replacements) -> Item {
    let replacements = env.replacements.push(replacements);
    let namespace = env.insert_namespace(Namespace::Replacing {
        base: base.namespace,
        replacements,
    });
    let value = env.insert_value(Value::Replacing {
        base: base.value,
        replacements,
    });
    Item { namespace, value }
}
