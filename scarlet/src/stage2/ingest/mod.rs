use super::structure::{Environment, Item};
use crate::stage1::structure::expression::Expression;

mod postfix;
mod root;

pub fn ingest_expression(env: &mut Environment, mut expression: Expression) -> Item {
    let result = if let Some(post) = expression.others.pop() {
        postfix::ingest(env, expression, post)
    } else {
        root::ingest(env, expression.root)
    };
    result
}
