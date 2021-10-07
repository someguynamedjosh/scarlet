use crate::{
    stage1::structure::{construct::Construct, expression::Expression},
    stage2::{
        ingest_expression,
        structure::{Environment, ItemId},
    },
};

mod member;
mod substituting;
mod matchh;

pub fn ingest(env: &mut Environment, remainder: Expression, post: Construct) -> ItemId {
    let base = ingest_expression(env, remainder);
    match &post.label[..] {
        "matching" => matchh::ingest(env, base, post),
        "member" => member::ingest(env, base, post),
        "substituting" => substituting::ingest(env, base, post),
        "type_is" => todo!(),
        _ => todo!("nice error"),
    }
}
