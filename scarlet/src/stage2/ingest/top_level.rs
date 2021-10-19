use std::collections::HashMap;

use super::from_tree;
use crate::{
    stage1::structure::{Module, TokenTree},
    stage2::{
        ingest::util,
        structure::{Environment, ItemId},
    },
};

pub fn ingest_tree<'x>(
    src: &'x TokenTree<'x>,
    env: &mut Environment<'x>,
    in_scopes: &[&HashMap<&str, ItemId<'x>>],
) -> ItemId<'x> {
    let into = util::begin_item(src, env);
    ingest_tree_into(src, env, into, in_scopes);
    into
}

pub fn ingest_tree_into<'x>(
    src: &'x TokenTree<'x>,
    env: &mut Environment<'x>,
    into: ItemId<'x>,
    in_scopes: &[&HashMap<&str, ItemId<'x>>],
) {
    let definition = from_tree::definition_from_tree(src, env, in_scopes);
    env.items.get_mut(into).definition = Some(definition);
}

pub fn ingest_module<'x>(
    src: &'x Module,
    env: &mut Environment<'x>,
    into: ItemId<'x>,
    in_scopes: &[&HashMap<&str, ItemId<'x>>],
) {
    let mut children = Vec::new();
    for (name, module) in &src.children {
        assert_eq!(module.self_content.len(), 1);
        let src = &module.self_content[0];
        children.push((&name[..], module, util::begin_item(src, env)));
    }

    let scope_map: HashMap<_, _> = children.iter().map(|(name, _, id)| (*name, *id)).collect();
    let new_scopes = util::with_extra_scope(in_scopes, &scope_map);

    assert_eq!(src.self_content.len(), 1);
    ingest_tree_into(&src.self_content[0], env, into, in_scopes);

    for (_, src, id) in children {
        ingest_module(src, env, id, &new_scopes[..]);
    }
}

pub fn ingest<'x>(src: &'x Module) -> Environment<'x> {
    assert_eq!(src.self_content.len(), 1);
    let mut env = Environment::new();
    let into = util::begin_item(&src.self_content[0], &mut env);
    ingest_module(src, &mut env, into, &[]);
    env
}
