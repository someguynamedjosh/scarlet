use crate::{
    stage1::structure::{construct::Construct, expression::Expression, statement::Statement},
    stage2::{
        self,
        structure::{Definitions, Item},
    },
};

pub fn ingest(remainder: Expression, post: Construct) -> Item {
    let base = Box::new(stage2::ingest(remainder));
    let definitions = ingest_definitions(post);
    Item::Defining { base, definitions }
}

fn ingest_definitions(post: Construct) -> Definitions {
    let mut definitions = Definitions::new();
    for statement in post.expect_statements("defining").unwrap() {
        ingest_definition(statement, &mut definitions)
    }
    definitions
}

fn ingest_definition(statement: &Statement, definitions: &mut Definitions) {
    match statement {
        Statement::Is(is) => {
            let name = is.name.expect_ident().expect("TODO error").to_owned();
            let item = stage2::ingest(is.value.clone());
            definitions.insert_no_replace(name, item);
        }
        _ => todo!("nice error"),
    }
}
