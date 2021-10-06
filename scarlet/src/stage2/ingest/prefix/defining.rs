use crate::{
    stage1::structure::{construct::Construct, expression::Expression, statement::Statement},
    stage2::{
        self,
        structure::{Definitions, Environment, Item, ItemId},
    },
};

pub fn ingest(env: &mut Environment, remainder: Expression, post: Construct) -> ItemId {
    let base = stage2::ingest_expression(env, remainder);
    let definitions = ingest_definitions(env, post);
    env.push_item(Item::Defining { base, definitions })
}

fn ingest_definitions(env: &mut Environment, post: Construct) -> Definitions {
    let mut definitions = Definitions::new();
    for statement in post.expect_statements("defining").unwrap() {
        ingest_definition(env, statement, &mut definitions)
    }
    definitions
}

fn ingest_definition(env: &mut Environment, statement: &Statement, definitions: &mut Definitions) {
    match statement {
        Statement::Expression(expr) => {
            let mut expr = expr.clone();
            let name = expr
                .extract_target()
                .expect("TODO: nice error")
                .expect("TODO: anonymous values");
            let name = name.expect_ident().expect("TODO: nice error").to_owned();
            let item = stage2::ingest_expression(env, expr);
            definitions.insert_no_replace(name, item);
        }
        _ => todo!("nice error, unexpected {:?}", statement),
    }
}
