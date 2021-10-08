use crate::{
    stage1::structure::construct::Construct,
    stage2::{
        self,
        structure::{Environment, Item, ItemId},
    },
};

pub fn ingest(env: &mut Environment, base: ItemId, post: Construct) -> ItemId {
    let mut cases = Vec::new();
    for expr in post.expect_expressions("matching").unwrap() {
        let mut expr = expr.clone();
        let case = expr
            .extract_single_expression("on")
            .expect("TODO: Nice error")
            .expect("TODO: Nice error");
        let case = stage2::ingest_expression(env, case);
        let expr = stage2::ingest_expression(env, expr);
        cases.push((case, expr));
    }
    let cases2 = cases.clone();
    let result = env.push_item(Item::Match { base, cases });
    env.set_parent_scope(base, result);
    for (condition, value) in cases2 {
        env.set_parent_scope(condition, result);
        env.set_parent_scope(value, result);
    }
    result
}
