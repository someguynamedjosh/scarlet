use crate::{
    stage1::structure::{construct::Construct, statement::Statement},
    stage2::{
        self,
        structure::{Environment, Item, Substitutions},
    },
};

pub fn ingest(env: &mut Environment, base: Item, post: Construct) -> Item {
    let base = Box::new(base);
    let substitutions = ingest_substitutions(env, post);
    Item::Substituting {
        base,
        substitutions,
    }
}

fn ingest_substitutions(env: &mut Environment, post: Construct) -> Substitutions {
    let mut substitutions = Substitutions::new();
    for statement in post.expect_statements("substituting").unwrap() {
        ingest_substitution(env, statement, &mut substitutions);
    }
    substitutions
}

fn ingest_substitution(
    env: &mut Environment,
    statement: &Statement,
    substitutions: &mut Substitutions,
) {
    match statement {
        Statement::Is(is) => {
            let target = stage2::ingest_expression(env, is.name.clone());
            let value = stage2::ingest_expression(env, is.value.clone());
            substitutions.push((target, value));
        }
        Statement::Expression(..) => todo!(),
        _ => todo!("nice error"),
    }
}
