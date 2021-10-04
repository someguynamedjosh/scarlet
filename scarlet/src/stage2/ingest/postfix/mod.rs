use crate::{
    stage1::structure::{construct::Construct, expression::Expression},
    stage2::{
        self,
        structure::{Environment, Item},
    },
};

mod defining;
mod from_values;
mod member;
mod substituting;

pub fn ingest(env: &mut Environment, remainder: Expression, post: Construct) -> Item {
    if post.label == "defining" {
        defining::ingest(env, remainder, post)
    } else {
        let base = stage2::ingest_expression(env, remainder);
        ingest_non_defining(env, base, post)
    }
}

fn ingest_non_defining(env: &mut Environment, base: Item, post: Construct) -> Item {
    match &post.label[..] {
        "defining" => unreachable!(),
        "FromValues" => from_values::ingest(env, base, post),
        "member" => member::ingest(base, post),
        "substituting" => substituting::ingest(env, base, post),
        "type_is" => todo!(),
        _ => todo!("nice error"),
    }
}
