use super::structure::Item;
use crate::stage1::structure::expression::Expression;

mod postfix;
mod root;

pub fn ingest(mut expression: Expression) -> Item {
    let result = if let Some(post) = expression.others.pop() {
        postfix::ingest(expression, post)
    } else {
        root::ingest(expression.root)
    };
    result
}
