use crate::{
    stage1::structure::construct::Construct,
    stage2::{
        self,
        structure::{Environment, Item, ItemId, Substitutions},
    },
};

pub fn ingest(env: &mut Environment, base: ItemId, post: Construct) -> ItemId {
    // TODO: Nice errors.
    let exprs = post.expect_expressions("substituting").unwrap().clone();
    let mut substitutions = Substitutions::new();
    for expr in exprs {
        let mut expr = expr.clone();
        let target = if let Some(target) = expr.extract_target() {
            Some(stage2::ingest_expression(env, target.unwrap()))
        } else {
            None
        };
        let value = stage2::ingest_expression(env, expr.clone());
        substitutions.push((target, value));
    }
    let result = env.push_item(Item::Substituting {
        base,
        substitutions: substitutions.clone(),
    });
    env.set_parent_scope(base, result);
    for (target, value) in substitutions {
        target.map(|t| env.set_parent_scope(t, result));
        env.set_parent_scope(value, result);
    }
    result
}
