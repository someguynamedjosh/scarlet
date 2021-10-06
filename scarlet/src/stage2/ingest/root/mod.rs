use crate::{
    stage1::structure::construct::Construct,
    stage2::structure::{Environment, ItemId},
};

mod any;
mod builtin_item;
mod identifier;
mod u8;
mod variant;

pub fn ingest(env: &mut Environment, root: Construct) -> ItemId {
    match &root.label[..] {
        "any" => any::ingest(env, root),
        "builtin_item" => builtin_item::ingest(env, root),
        "identifier" => identifier::ingest(env, root),
        "u8" => u8::ingest(env, root),
        "variant_of" => variant::ingest(env, root),
        _ => todo!("Nice error"),
    }
}
