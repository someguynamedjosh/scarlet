use crate::{
    stage1::structure::construct::Construct,
    stage2::{
        self,
        structure::{Environment, Item, ItemId},
    },
};

pub fn ingest(env: &mut Environment, base: ItemId, post: Construct) -> ItemId {
    // TODO: Nice errors.
    let exprs = post.expect_expressions("substituting").unwrap().clone();
    let mut result = base;
    for expr in exprs {
        let mut expr = expr.clone();
        let target = stage2::ingest_expression(env, expr.extract_target().unwrap().unwrap());
        let value = stage2::ingest_expression(env, expr.clone());
        result = env.push_item(Item::Substituting {
            base: result,
            target,
            value,
        })
    }
    result
}
