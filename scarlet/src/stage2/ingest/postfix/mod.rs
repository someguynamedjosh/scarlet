use crate::{
    stage1::structure::{construct::Construct, expression::Expression},
    stage2::{
        ingest_expression,
        structure::{Environment, Item, ItemId},
    },
};

mod matchh;
mod member;
mod substituting;

pub fn ingest(env: &mut Environment, remainder: Expression, post: Construct) -> ItemId {
    let base = ingest_expression(env, remainder);
    match &post.label[..] {
        "displayed" => ingest_displayed(env, base, post),
        "matching" => matchh::ingest(env, base, post),
        "member" => member::ingest(env, base, post),
        "substituting" => substituting::ingest(env, base, post),
        "type_is" => ingest_type_is(env, base, post),
        _ => todo!("nice error"),
    }
}

pub fn ingest_displayed(env: &mut Environment, base: ItemId, _post: Construct) -> ItemId {
    env.mark_displayed(base);
    base
}

pub fn ingest_type_is(env: &mut Environment, base: ItemId, post: Construct) -> ItemId {
    let typee = post
        .expect_single_expression("type_is")
        .expect("TODO: Nice error");
    let typee = ingest_expression(env, typee.clone());
    let result = env.push_item(Item::TypeIs { base, typee });
    env.set_parent_scope(base, result);
    env.set_parent_scope(typee, result);
    result
}
