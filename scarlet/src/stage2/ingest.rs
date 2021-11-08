use std::collections::HashMap;

use super::structure::{Environment, Item, ItemId};
use crate::{stage1::structure::Module, stage2::structure::Definition};

pub fn ingest<'x>(src: &'x Module) -> (Environment<'x>, ItemId<'x>) {
    let mut env = Environment::new();
    let root = env.items.push(Item {
        cached_reduction: None,
        definition: Some(Definition::Resolvable(&src.self_content)),
        dependencies: None,
        scope: HashMap::default(),
        shown_from: vec![],
        original_definition: &src.self_content,
    });
    let root = env.reduce(root);
    env.get_deps(root);
    (env, root)
}
