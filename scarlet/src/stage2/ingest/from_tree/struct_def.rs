use std::collections::HashMap;

use crate::{
    stage1::structure::TokenTree,
    stage2::{
        ingest::{top_level, util},
        structure::{Definition, Environment, ItemId, StructField},
    },
};

pub fn ingest<'x>(
    body: &'x Vec<TokenTree<'x>>,
    in_scopes: &[&HashMap<&str, ItemId<'x>>],
    env: &mut Environment<'x>,
) -> Definition<'x> {
    let fields: Vec<_> = body.iter().map(util::maybe_target).collect();
    let ids: Vec<_> = fields
        .iter()
        .map(|target| util::begin_item(&target.value, env))
        .collect();
    let mut scope_map = HashMap::new();
    for (field, id) in fields.iter().zip(ids.iter()) {
        if let Some((_, name)) = &field.target {
            scope_map.insert(*name, *id);
        }
    }
    let new_scopes = util::with_extra_scope(in_scopes, &scope_map);
    for (field, id) in fields.iter().zip(ids.iter()) {
        top_level::ingest_tree_into(field.value, env, *id, &new_scopes[..]);
    }
    let mut labeled_fields = Vec::new();
    for (field, id) in fields.iter().zip(ids.iter()) {
        let name = field.target.clone().map(|x| x.1.to_owned());
        labeled_fields.push(StructField { name, value: *id });
    }
    Definition::Struct(labeled_fields)
}
