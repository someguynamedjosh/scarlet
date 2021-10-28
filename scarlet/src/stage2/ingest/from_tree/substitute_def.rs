use std::collections::HashMap;

use crate::{
    stage1::structure::{Token, TokenTree},
    stage2::{
        ingest::top_level,
        structure::{Definition, Environment, ItemId, UnresolvedSubstitution},
    },
};

pub fn ingest<'x>(
    body: &'x Vec<TokenTree<'x>>,
    env: &mut Environment<'x>,
    in_scopes: &[&HashMap<Token<'x>, ItemId<'x>>],
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
                let target_name = match target {
                    &TokenTree::Token(token) => Some(token),
                    _ => None,
                };
                let target_meaning = Some(top_level::ingest_tree(&target, env, in_scopes));
                let value = top_level::ingest_tree(&body[1], env, in_scopes);
                substitutions.push(UnresolvedSubstitution {
                    target_name,
                    target_meaning,
                    value,
                })
            }
            _ => {
                let target_name = None;
                let target_meaning = None;
                let value = top_level::ingest_tree(item, env, in_scopes);
                substitutions.push(UnresolvedSubstitution {
                    target_name,
                    target_meaning,
                    value,
                })
            }
        }
    }
    Definition::UnresolvedSubstitute(base, substitutions)
}
