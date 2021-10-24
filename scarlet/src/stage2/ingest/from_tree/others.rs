use std::collections::HashMap;

use crate::{
    stage1::structure::{Token, TokenTree},
    stage2::{
        ingest::top_level,
        structure::{After, BuiltinValue, Definition, Environment, ItemId, Variable},
    },
};

pub fn after_def<'x>(
    body: &'x Vec<TokenTree<'x>>,
    env: &mut Environment<'x>,
    in_scopes: &[&HashMap<Token<'x>, ItemId<'x>>],
) -> Definition<'x> {
    if body.len() != 2 {
        todo!("Nice error");
    }

    let base = top_level::ingest_tree(&body[1], env, in_scopes);

    let vals = body[0]
        .unwrap_builtin("vals")
        .iter()
        .map(|tt| top_level::ingest_tree(tt, env, in_scopes))
        .collect();

    Definition::After { base, vals }
}

pub fn member_def<'x>(
    body: &'x Vec<TokenTree<'x>>,
    env: &mut Environment<'x>,
    in_scopes: &[&HashMap<Token<'x>, ItemId<'x>>],
) -> Definition<'x> {
    assert_eq!(body.len(), 2);
    let base = &body[0];
    let base = top_level::ingest_tree(base, env, in_scopes);
    let name = body[1].as_token().unwrap().to_owned();
    Definition::Member(base, name)
}

pub fn show<'x>(
    body: &'x Vec<TokenTree<'x>>,
    env: &mut Environment<'x>,
    in_scopes: &[&HashMap<Token<'x>, ItemId<'x>>],
    into: ItemId<'x>,
) -> Definition<'x> {
    if body.len() != 1 {
        todo!("Nice error");
    }
    let value = &body[0];
    let value = top_level::ingest_tree(value, env, in_scopes);
    env.items[value].shown_from.push(into);
    Definition::Other(value)
}

pub fn token_def<'x>(
    token: &&str,
    in_scopes: &[&HashMap<Token<'x>, ItemId<'x>>],
) -> Definition<'x> {
    if let Ok(num) = token.parse() {
        Definition::BuiltinValue(BuiltinValue::_32U(num))
    } else if token == &"true" {
        Definition::BuiltinValue(BuiltinValue::Bool(true))
    } else if token == &"false" {
        Definition::BuiltinValue(BuiltinValue::Bool(false))
    } else {
        let mut result = None;
        // Reversed so we search more local scopes first.
        for scope in in_scopes.iter().rev() {
            if let Some(id) = scope.get(token) {
                result = Some(*id);
                break;
            }
        }
        let id = result.expect(&format!("TODO: Nice error, bad ident {}", token));
        Definition::Other(id)
    }
}

pub fn variable_def<'x>(
    body: &'x Vec<TokenTree<'x>>,
    env: &mut Environment<'x>,
    in_scopes: &[&HashMap<Token<'x>, ItemId<'x>>],
) -> Definition<'x> {
    if body.len() != 1 {
        todo!("Nice error");
    }
    let pattern = &body[0];
    let pattern = top_level::ingest_tree(pattern, env, in_scopes);
    let var = env.vars.push(Variable { pattern });
    Definition::Variable(var)
}
