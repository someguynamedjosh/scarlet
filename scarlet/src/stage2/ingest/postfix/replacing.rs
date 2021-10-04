use crate::{
    stage1::structure::{construct::Construct, statement::Statement},
    stage2::{
        self,
        structure::{Environment, Item, Replacements},
    },
};

pub fn ingest(env: &mut Environment, base: Item, post: Construct) -> Item {
    let base = Box::new(base);
    let replacements = ingest_replacements(env, post);
    Item::Replacing { base, replacements }
}

fn ingest_replacements(env: &mut Environment, post: Construct) -> Replacements {
    let mut replacements = Replacements::new();
    for statement in post.expect_statements("replacing").unwrap() {
        ingest_replacement(env, statement, &mut replacements);
    }
    replacements
}

fn ingest_replacement(
    env: &mut Environment,
    statement: &Statement,
    replacements: &mut Replacements,
) {
    match statement {
        Statement::Replace(replace) => {
            let target = stage2::ingest_expression(env, replace.target.clone());
            let value = stage2::ingest_expression(env, replace.value.clone());
            replacements.push((target, value));
        }
        Statement::Expression(..) => todo!(),
        _ => todo!("nice error"),
    }
}
