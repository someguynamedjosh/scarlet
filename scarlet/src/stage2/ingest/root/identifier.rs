use crate::{
    stage1::structure::construct::Construct,
    stage2::structure::{Environment, Item, ItemId},
};

pub fn ingest(env: &mut Environment, root: Construct) -> ItemId {
    let name = root.expect_ident().unwrap().to_owned();
    env.push_item(Item::Identifier(name))
}
