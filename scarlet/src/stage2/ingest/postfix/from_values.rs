use crate::{
    stage1::structure::construct::Construct,
    stage2::{
        self,
        structure::{Environment, Item, ItemId},
    },
};

pub fn ingest(env: &mut Environment, base: ItemId, post: Construct) -> ItemId {
    let values = ingest_from_dependants(env, post);
    env.push_item(Item::From { base, values })
}

fn ingest_from_dependants(env: &mut Environment, post: Construct) -> Vec<ItemId> {
    let items = post.expect_statements("FromValues").unwrap();
    let items = items
        .iter()
        .map(|i| i.expect_expression().expect("TODO: Nice error"));
    let items = items.map(|i| stage2::ingest_expression(env, i.clone()));
    items.collect()
}
