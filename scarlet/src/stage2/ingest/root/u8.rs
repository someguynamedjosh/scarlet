use crate::{
    stage1::structure::construct::Construct,
    stage2::structure::{BuiltinValue, Environment, Item, ItemId},
};

pub fn ingest(env: &mut Environment, root: Construct) -> ItemId {
    let value = root.expect_text("u8").unwrap().parse().unwrap();
    env.push_item(Item::BuiltinValue(BuiltinValue::U8(value)))
}
