use crate::{
    stage1::structure::{construct::Construct, expression::Expression},
    stage2::{self, structure::Item},
};

mod defining;
mod from_values;
mod member;
mod replacing;

pub fn ingest(remainder: Expression, post: Construct) -> Item {
    if post.label == "defining" {
        defining::ingest(remainder, post)
    } else {
        let base = stage2::ingest(remainder);
        ingest_non_defining(base, post)
    }
}

fn ingest_non_defining(base: Item, post: Construct) -> Item {
    match &post.label[..] {
        "defining" => unreachable!(),
        "FromValues" => from_values::ingest(base, post),
        "member" => member::ingest(base, post),
        "replacing" => replacing::ingest(base, post),
        "type_is" => todo!(),
        _ => todo!("nice error"),
    }
}
