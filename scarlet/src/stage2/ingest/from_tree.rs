mod match_def;
mod others;
mod struct_def;
mod substitute_def;
mod using;

use std::{collections::HashMap, marker::PhantomData};

use super::util::begin_item;
use crate::{
    stage1::structure::{Token, TokenTree},
    stage2::{
        ingest::top_level,
        structure::{BuiltinOperation, BuiltinPattern, Definition, Environment, ItemId, Variable},
    },
};

pub fn definition_from_tree<'x>(
    src: &'x TokenTree<'x>,
    env: &mut Environment<'x>,
    in_scopes: &[&HashMap<Token<'x>, ItemId<'x>>],
    into: ItemId<'x>,
) -> Definition<'x> {
    match src {
        TokenTree::Token(token) => others::token_def(token, in_scopes),

        TokenTree::BuiltinRule {
            name: "after",
            body,
        } => others::after_def(body, env, in_scopes),

        TokenTree::BuiltinRule { name: "any", body } => others::variable_def(body, env, in_scopes),
        TokenTree::BuiltinRule {
            name: "match",
            body,
        } => match_def::ingest(body, env, in_scopes),
        TokenTree::BuiltinRule {
            name: "matches",
            body,
        } => match_def::ingest_matches(body, env, in_scopes),
        TokenTree::BuiltinRule {
            name: "member",
            body,
        } => others::member_def(body, env, in_scopes),
        TokenTree::BuiltinRule { name: "show", body } => others::show(body, env, in_scopes, into),
        TokenTree::BuiltinRule {
            name: "struct",
            body,
        } => struct_def::ingest(body, in_scopes, env),
        TokenTree::BuiltinRule {
            name: "substitute",
            body,
        } => substitute_def::ingest(body, env, in_scopes),
        TokenTree::BuiltinRule {
            name: "using",
            body,
        } => using::ingest(body, env, in_scopes),

        TokenTree::BuiltinRule {
            name: "PATTERN",
            body,
        } => builtin_pattern_def(BuiltinPattern::God, src, body, env, in_scopes),
        TokenTree::BuiltinRule { name: "32U", body } => {
            builtin_pattern_def(BuiltinPattern::_32U, src, body, env, in_scopes)
        }
        TokenTree::BuiltinRule { name: "BOOL", body } => {
            builtin_pattern_def(BuiltinPattern::Bool, src, body, env, in_scopes)
        }
        TokenTree::BuiltinRule { name: "AND", body } => and_pattern_def(src, body, env, in_scopes),

        TokenTree::BuiltinRule {
            name: "sum_32u",
            body,
        } => builtin_op_def(BuiltinOperation::Sum32U, body, env, in_scopes),
        TokenTree::BuiltinRule {
            name: "dif_32u",
            body,
        } => builtin_op_def(BuiltinOperation::Dif32U, body, env, in_scopes),

        TokenTree::BuiltinRule { name, .. } => todo!("Nice error, unrecognized builtin {}", name),
    }
}

fn builtin_op_def<'x>(
    op: BuiltinOperation,
    body: &'x Vec<TokenTree<'x>>,
    env: &mut Environment<'x>,
    in_scopes: &[&HashMap<Token<'x>, ItemId<'x>>],
) -> Definition<'x> {
    let args: Vec<_> = body
        .iter()
        .map(|tt| top_level::ingest_tree(tt, env, in_scopes))
        .collect();
    Definition::BuiltinOperation(op, args)
}

fn and_pattern_def<'x>(
    src: &'x TokenTree<'x>,
    body: &'x Vec<TokenTree<'x>>,
    env: &mut Environment<'x>,
    in_scopes: &[&HashMap<Token<'x>, ItemId<'x>>],
) -> Definition<'x> {
    assert_eq!(body.len(), 2);
    let left = top_level::ingest_tree(&body[0], env, in_scopes);
    let right = top_level::ingest_tree(&body[1], env, in_scopes);
    builtin_pattern_def(BuiltinPattern::And(left, right), src, body, env, in_scopes)
}

fn builtin_pattern_def<'x>(
    builtin_pattern: BuiltinPattern<'x>,
    src: &'x TokenTree<'x>,
    _body: &'x Vec<TokenTree<'x>>,
    env: &mut Environment<'x>,
    in_scopes: &[&HashMap<Token<'x>, ItemId<'x>>],
) -> Definition<'x> {
    // assert_eq!(
    //     body.len(),
    //     0,
    //     "TODO: Nice error, expected zero argument to builtin."
    // );
    let matches = begin_item(src, env, in_scopes);
    env.items[matches].definition = Some(Definition::BuiltinPattern(builtin_pattern));
    let var = Variable { pd: PhantomData };
    let var = env.vars.push(var);
    Definition::Variable { var, matches }
}
