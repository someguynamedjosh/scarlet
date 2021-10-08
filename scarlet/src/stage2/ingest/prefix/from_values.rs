use crate::{
    stage1::structure::construct::Construct,
    stage2::{
        self,
        structure::{Environment, Item, ItemId},
    },
};

pub fn ingest(env: &mut Environment, base: ItemId, post: Construct) -> ItemId {
    let value = post.expect_single_expression("FromValues").unwrap();
    let value = stage2::ingest_expression(env, value.clone());
    let result = env.push_item(Item::From { base, value });
    env.set_parent_scope(base, result);
    env.set_parent_scope(value, result);
    result
}
