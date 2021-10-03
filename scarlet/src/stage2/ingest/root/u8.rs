use crate::{
    stage1::structure::construct::Construct,
    stage2::structure::{BuiltinValue, Item},
};

pub fn ingest(root: Construct) -> Item {
    let value = root.expect_text("u8").unwrap().parse().unwrap();
    Item::BuiltinValue(BuiltinValue::U8(value))
}
