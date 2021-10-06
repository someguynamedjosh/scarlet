use super::structure::{Environment, Item, ItemId};
use crate::stage1::structure::expression::Expression;

mod postfix;
mod root;

pub fn ingest_expression(env: &mut Environment, mut expression: Expression) -> ItemId {
    let result = if let Some(pre) = expression.pres.pop() {
        // prefix::ingest(env, expression, pre)
        todo!()
    } else if let Some(post) = expression.posts.pop() {
        postfix::ingest(env, expression, post)
    } else {
        root::ingest(env, expression.root)
    };
    result
}
