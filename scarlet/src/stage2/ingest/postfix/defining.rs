use crate::{
    stage1::structure::{construct::Construct, expression::Expression, statement::Statement},
    stage2::{
        self,
        structure::{Definitions, Environment, Item, Namespace, NamespaceId},
    },
};

pub fn ingest(
    env: &mut Environment,
    remainder: Expression,
    post: Construct,
    in_namespace: NamespaceId,
) -> Item {
    let this_namespace = env.new_undefined_namespace();
    let base = stage2::ingest(env, remainder, this_namespace);
    let definitions = ingest_definitions(env, post, this_namespace);
    env.define_namespace(
        this_namespace,
        Namespace::Defining {
            base: base.namespace,
            definitions,
            parent: in_namespace,
        },
    );
    Item {
        namespace: this_namespace,
        value: base.value,
    }
}

fn ingest_definitions(
    env: &mut Environment,
    post: Construct,
    this_namespace: NamespaceId,
) -> Definitions {
    let mut definitions = Definitions::new();
    for statement in post.expect_statements("defining").unwrap() {
        ingest_definition(env, statement, this_namespace, &mut definitions)
    }
    definitions
}

fn ingest_definition(
    env: &mut Environment,
    statement: &Statement,
    this_namespace: NamespaceId,
    definitions: &mut Definitions,
) {
    match statement {
        Statement::Is(is) => {
            let name = is.name.expect_ident().expect("TODO error").to_owned();
            let item = stage2::ingest(env, is.value.clone(), this_namespace);
            definitions.insert_no_replace(name, item);
        }
        _ => todo!("nice error"),
    }
}
