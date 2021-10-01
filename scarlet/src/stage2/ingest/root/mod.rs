use crate::{
    stage1::structure::construct::Construct,
    stage2::structure::{Environment, Item, NamespaceId},
};

mod any;
mod builtin_item;
mod identifier;
mod u8;
mod variant;

pub fn ingest(env: &mut Environment, root: Construct, in_namespace: NamespaceId) -> Item {
    match &root.label[..] {
        "any" => any::ingest(env, root, in_namespace),
        "builtin_item" => builtin_item::ingest(env, root, in_namespace),
        "identifier" => identifier::ingest(env, root, in_namespace),
        "u8" => u8::ingest(env, root, in_namespace),
        "variant" => variant::ingest(env, root, in_namespace),
        _ => todo!("Nice error"),
    }
}
