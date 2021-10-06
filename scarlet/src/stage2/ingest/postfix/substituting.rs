use crate::{
    stage1::structure::construct::Construct,
    stage2::{
        self,
        structure::{Environment, Item, ItemId},
    },
};

pub fn ingest(env: &mut Environment, base: ItemId, post: Construct) -> ItemId {
    // TODO: Nice errors.
    let mut expr = post
        .expect_single_expression("substituting")
        .unwrap()
        .clone();
    let target = stage2::ingest_expression(env, expr.extract_target().unwrap().unwrap());
    let value = stage2::ingest_expression(env, expr.clone());
    env.push_item(Item::Substituting {
        base,
        target,
        value,
    })
}
