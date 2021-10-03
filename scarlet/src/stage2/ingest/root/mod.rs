use crate::{stage1::structure::construct::Construct, stage2::structure::Item};

mod any;
mod builtin_item;
mod identifier;
mod u8;
mod variant;

pub fn ingest(root: Construct) -> Item {
    match &root.label[..] {
        "any" => any::ingest(root),
        "builtin_item" => builtin_item::ingest(root),
        "identifier" => identifier::ingest(root),
        "u8" => u8::ingest(root),
        "variant" => variant::ingest(root),
        _ => todo!("Nice error"),
    }
}
