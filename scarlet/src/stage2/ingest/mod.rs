use super::structure::{Environment, Item, ItemId};
use crate::stage1::structure::{construct::Construct, expression::Expression};

mod postfix;
mod prefix;
mod root;

fn pop_first(from: &mut Vec<Construct>) -> Option<Construct> {
    if from.len() > 0 {
        Some(from.remove(0))
    } else {
        None
    }
}

pub fn ingest_expression(env: &mut Environment, mut expression: Expression) -> ItemId {
    if let Some(pre) = pop_first(&mut expression.pres) {
        prefix::ingest(env, expression, pre)
    } else if let Some(post) = expression.posts.pop() {
        postfix::ingest(env, expression, post)
    } else {
        root::ingest(env, expression.root)
    }
}
