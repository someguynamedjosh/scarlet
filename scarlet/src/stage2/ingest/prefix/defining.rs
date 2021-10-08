use crate::{
    stage1::structure::{construct::Construct, expression::Expression},
    stage2::{
        self,
        structure::{Definitions, Environment, Item, ItemId},
    },
};

pub fn ingest(env: &mut Environment, remainder: Expression, post: Construct) -> ItemId {
    let base = stage2::ingest_expression(env, remainder);
    let definitions = ingest_definitions(env, post);
    let definitions2 = definitions.clone();
    let result = env.push_item(Item::Defining { base, definitions });
    env.set_parent_scope(base, result);
    for (_, def) in definitions2 {
        env.set_parent_scope(def, result);
    }
    result
}

fn ingest_definitions(env: &mut Environment, post: Construct) -> Definitions {
    let mut definitions = Definitions::new();
    for expression in post.expect_expressions("defining").unwrap() {
        ingest_definition(env, expression, &mut definitions)
    }
    definitions
}

fn ingest_definition(
    env: &mut Environment,
    expression: &Expression,
    definitions: &mut Definitions,
) {
    let mut expr = expression.clone();
    let name = expr
        .extract_target()
        .expect("TODO: nice error")
        .expect("TODO: anonymous values");
    let name = name.expect_ident().expect("TODO: nice error").to_owned();
    let item = stage2::ingest_expression(env, expr);
    definitions.insert_no_replace(name, item);
}
