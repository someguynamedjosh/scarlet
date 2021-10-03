use crate::{
    stage1::structure::{construct::Construct, statement::Statement},
    stage2::{
        self,
        structure::{Item, Replacements},
    },
};

pub fn ingest(base: Item, post: Construct) -> Item {
    let base = Box::new(base);
    let replacements = ingest_replacements(post);
    Item::Replacing { base, replacements }
}

fn ingest_replacements(post: Construct) -> Replacements {
    let mut replacements = Replacements::new();
    for statement in post.expect_statements("replacing").unwrap() {
        ingest_replacement(statement, &mut replacements);
    }
    replacements
}

fn ingest_replacement(statement: &Statement, replacements: &mut Replacements) {
    match statement {
        Statement::Replace(replace) => {
            let target = stage2::ingest(replace.target.clone());
            let value = stage2::ingest(replace.value.clone());
            replacements.push((target, value));
        }
        Statement::Expression(..) => todo!(),
        _ => todo!("nice error"),
    }
}
