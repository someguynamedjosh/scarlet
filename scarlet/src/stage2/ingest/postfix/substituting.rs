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
        let target = if let Some(target) = expr.extract_target() {
            Some(stage2::ingest_expression(env, target.unwrap()))
        } else {
            None
        };
        let value = stage2::ingest_expression(env, expr.clone());
        let next_result = env.push_item(Item::Substituting {
            base: result,
            target,
            value,
        });
        env.set_parent_scope(result, next_result);
        if let Some(target) = target {
            env.set_parent_scope(target, next_result);
        }
        env.set_parent_scope(value, next_result);
        result = next_result;
    }
    result
}
