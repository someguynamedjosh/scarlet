use std::collections::HashMap;

use crate::{
    stage1::structure::TokenTree,
    stage2::{
        ingest::top_level,
        structure::{Definition, Environment, ItemId, Substitution},
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
    let substitution_source = body[1].unwrap_builtin("substitutions");
    let mut substitutions = Vec::new();
    for item in substitution_source {
        match item {
            TokenTree::BuiltinRule {
                name: "target",
                body,
            } => {
                assert_eq!(body.len(), 2);
                let target = &body[0];
                let target = top_level::ingest_tree(&target, env, in_scopes);
                let target = Some(target);
                let value = top_level::ingest_tree(&body[1], env, in_scopes);
                substitutions.push(Substitution { target, value })
            }
            _ => {
                let target = None;
                let value = top_level::ingest_tree(item, env, in_scopes);
                substitutions.push(Substitution { target, value })
            }
        }
    }
    Definition::Substitute(base, substitutions)
}
