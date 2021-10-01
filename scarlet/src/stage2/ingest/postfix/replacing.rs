use crate::{
    stage1::structure::{construct::Construct, statement::Statement},
    stage2::{
        self,
        structure::{Environment, Item, Namespace, NamespaceId, Replacements, Value},
    },
};

pub fn ingest(
    env: &mut Environment,
    base: Item,
    post: Construct,
    in_namespace: NamespaceId,
) -> Item {
    let mut replacements = Replacements::new();
    for statement in post.expect_statements("replacing").unwrap() {
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
