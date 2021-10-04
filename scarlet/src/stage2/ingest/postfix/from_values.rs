use crate::{
    stage1::structure::construct::Construct,
    stage2::{
        self,
        structure::{Environment, Item},
    },
};

pub fn ingest(env: &mut Environment, base: Item, post: Construct) -> Item {
    let values = ingest_from_dependants(env, post);
    from_item(base, values)
}

fn from_item(base: Item, values: Vec<Item>) -> Item {
    Item::From {
        base: Box::new(base),
        values,
    }
}

fn ingest_from_dependants(env: &mut Environment, post: Construct) -> Vec<Item> {
    let items = post.expect_statements("FromValues").unwrap();
    let items = items
        .iter()
        .map(|i| i.expect_expression().expect("TODO: Nice error"));
    let items = items.map(|i| stage2::ingest_expression(env, i.clone()));
    items.collect()
}
