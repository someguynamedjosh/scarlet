use std::collections::HashMap;

use crate::{
    stage1::structure::TokenTree,
    stage2::{
        ingest::top_level,
        structure::{Condition, Definition, Environment, ItemId},
    },
};

pub fn ingest<'x>(
    body: &'x Vec<TokenTree<'x>>,
    env: &mut Environment<'x>,
    in_scopes: &[&HashMap<&str, ItemId<'x>>],
) -> Definition<'x> {
    assert_eq!(body.len(), 2);
    let base = &body[0];
    let base = top_level::ingest_tree(base, env, in_scopes);
    let condition_source = body[1].unwrap_builtin("patterns");
    let mut conditions = Vec::new();
    let mut else_value = None;
    for item in condition_source {
        match item {
            TokenTree::BuiltinRule { name: "on", body } => {
                assert_eq!(body.len(), 2);
                let pattern = body[0].unwrap_builtin("pattern");
                assert_eq!(pattern.len(), 1);
                let pattern = top_level::ingest_tree(&pattern[0], env, in_scopes);
                let value = top_level::ingest_tree(&body[1], env, in_scopes);
                conditions.push(Condition { pattern, value })
            }
            TokenTree::BuiltinRule { name: "else", body } => {
                assert_eq!(body.len(), 1);
                let value = top_level::ingest_tree(&body[0], env, in_scopes);
                else_value = Some(value);
            }
            _ => unreachable!(),
        }
    }
    let else_value = else_value.expect("TODO: Nice error, no else specified.");
    Definition::Match {
        base,
        conditions,
        else_value,
    }
}
