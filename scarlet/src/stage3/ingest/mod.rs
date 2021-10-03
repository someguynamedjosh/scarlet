use self::context::Context;
use super::structure::Environment;
use crate::stage3::{ingest::dereference::ItemBeingDereferenced, structure::Item};

mod context;
mod dereference;
mod dereferenced_namespace;
mod dereferenced_value;
mod ingest_entry;
mod ingest_structures;

pub fn ingest(
    input: &mut crate::stage2::structure::Environment,
    root: crate::stage2::structure::Item,
) -> Environment {
    let mut env = Environment::new();
    let mut ctx = Context::new(input, &mut env);
    let root_namespace = ctx.ingest_namespace(root.namespace);
    let root_value = ctx.ingest_value(root.value);
    let new_root_item = Item {
        namespace: root_namespace,
        value: root_value,
    };
    println!("{:?}\nbecomes\n{:#?}\n", root, new_root_item);
    env
}
