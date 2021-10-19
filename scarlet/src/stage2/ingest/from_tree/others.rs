use std::collections::HashMap;

use crate::{
    stage1::structure::TokenTree,
    stage2::{
        ingest::{top_level, util},
        structure::{
            BuiltinValue, Condition, Definition, Environment, Item, ItemId, StructField,
            Substitution, Variable,
        },
    },
};

pub fn member_def<'x>(
    body: &'x Vec<TokenTree<'x>>,
    env: &mut Environment<'x>,
    in_scopes: &[&HashMap<&str, ItemId<'x>>],
) -> Definition<'x> {
    assert_eq!(body.len(), 2);
    let base = &body[0];
    let base = top_level::ingest_tree(base, env, in_scopes);
    let name = body[1].as_token().unwrap().to_owned();
    Definition::Member(base, name)
}

pub fn token_def<'x>(token: &&str, in_scopes: &[&HashMap<&str, ItemId<'x>>]) -> Definition<'x> {
    if let Ok(num) = token.parse() {
        Definition::BuiltinValue(BuiltinValue::U32(num))
    } else {
        let mut result = None;
        // Reversed so we search more local scopes first.
        for scope in in_scopes.iter().rev() {
            if let Some(id) = scope.get(token) {
                result = Some(*id);
                break;
            }
        }
        let id = result.expect("TODO: Nice error, bad ident");
        Definition::Other(id)
    }
}

pub fn variable_def<'x>(
    body: &'x Vec<TokenTree<'x>>,
    env: &mut Environment<'x>,
    in_scopes: &[&HashMap<&str, ItemId<'x>>],
) -> Definition<'x> {
    if body.len() != 1 {
        todo!("Nice error");
    }
    let pattern = &body[0];
    let pattern = top_level::ingest_tree(pattern, env, in_scopes);
    let var = env.vars.push(Variable { pattern });
    Definition::Variable(var)
}
