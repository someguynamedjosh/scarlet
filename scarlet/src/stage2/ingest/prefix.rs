use crate::{
    stage1::structure::{construct::Construct, expression::Expression},
    stage2::{
        ingest_expression,
        structure::{Environment, ItemId},
    },
};

mod defining;
mod from_values;

pub fn ingest(env: &mut Environment, remainder: Expression, post: Construct) -> ItemId {
    match &post.label[..] {
        "defining" => defining::ingest(env, remainder, post),
        "FromValues" => {
            let base = ingest_expression(env, remainder);
            from_values::ingest(env, base, post)
        }
        _ => todo!("nice error"),
    }
}
