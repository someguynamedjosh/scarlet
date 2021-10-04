use crate::{
    stage1::structure::construct::Construct,
    stage2::{
        self,
        structure::{Environment, Item, ItemId},
    },
};

pub fn ingest(env: &mut Environment, root: Construct) -> ItemId {
    let typee = root
        .expect_single_expression("any")
        .expect("TODO: Nice error");
    let typee = stage2::ingest_expression(env, typee.clone());
    let id = env.new_variable();
    env.push_item(Item::Any { typee, id })
}
