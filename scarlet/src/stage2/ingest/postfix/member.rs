use crate::{
    stage1::structure::construct::Construct,
    stage2::structure::{Environment, Item, ItemId},
};

pub fn ingest(env: &mut Environment, base: ItemId, post: Construct) -> ItemId {
    let the_name = ingest_ident_name(post);
    env.push_item(Item::Member {
        base,
        name: the_name,
    })
}

fn ingest_ident_name(post: Construct) -> String {
    let the_name = post
        .expect_single_expression("member")
        .expect("TODO: nice error")
        .expect_ident()
        .unwrap()
        .to_owned();
    the_name
}
