use crate::{stage1::structure::construct::Construct, stage2::structure::Item};

pub fn ingest(root: Construct) -> Item {
    let name = root.expect_ident().unwrap().to_owned();
    Item::Identifier(name)
}
