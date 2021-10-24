mod match_def;
mod others;
mod struct_def;
mod substitute_def;
mod using;

use std::collections::HashMap;

use crate::{
    stage1::structure::{Token, TokenTree},
    stage2::{
        ingest::top_level,
        structure::{BuiltinOperation, BuiltinValue, Definition, Environment, ItemId},
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
            name: "PATTERN", ..
        } => Definition::BuiltinValue(BuiltinValue::GodPattern),
        TokenTree::BuiltinRule { name: "32U", body } => {
            builtin_op_def(BuiltinOperation::_32UPattern, body, env, in_scopes)
        }
        TokenTree::BuiltinRule { name: "BOOL", body } => {
            builtin_op_def(BuiltinOperation::BoolPattern, body, env, in_scopes)
        }
        TokenTree::BuiltinRule {
            name: "sum_32u",
            body,
        } => builtin_op_def(BuiltinOperation::Sum32U, body, env, in_scopes),
        TokenTree::BuiltinRule {
            name: "dif_32u",
            body,
        } => builtin_op_def(BuiltinOperation::Dif32U, body, env, in_scopes),
        TokenTree::BuiltinRule {
            name: "matches",
            body,
        } => builtin_op_def(BuiltinOperation::Matches, body, env, in_scopes),

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
