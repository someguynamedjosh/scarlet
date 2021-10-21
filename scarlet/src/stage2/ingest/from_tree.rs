mod match_def;
mod others;
mod struct_def;
mod substitute_def;

use std::collections::HashMap;

use crate::{
    stage1::structure::TokenTree,
    stage2::{
        ingest::top_level,
        structure::{BuiltinOperation, BuiltinValue, Definition, Environment, ItemId},
    },
};

pub fn definition_from_tree<'x>(
    src: &'x TokenTree<'x>,
    env: &mut Environment<'x>,
    in_scopes: &[&HashMap<&str, ItemId<'x>>],
) -> Definition<'x> {
    match src {
        TokenTree::Token(token) => others::token_def(token, in_scopes),
        TokenTree::BuiltinRule {
            name: "match",
            body,
        } => match_def::ingest(body, env, in_scopes),
        TokenTree::BuiltinRule {
            name: "member",
            body,
        } => others::member_def(body, env, in_scopes),
        TokenTree::BuiltinRule {
            name: "struct",
            body,
        } => struct_def::ingest(body, in_scopes, env),
        TokenTree::BuiltinRule {
            name: "substitute",
            body,
        } => substitute_def::ingest(body, env, in_scopes),
        TokenTree::BuiltinRule { name: "any", body } => others::variable_def(body, env, in_scopes),

        TokenTree::BuiltinRule {
            name: "ANY_PATTERN",
            ..
        } => Definition::BuiltinValue(BuiltinValue::GodPattern),
        TokenTree::BuiltinRule { name: "32U", body } => {
            builtin_op_def(BuiltinOperation::_32UPattern, body, env, in_scopes)
        }
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
    in_scopes: &[&HashMap<&str, ItemId<'x>>],
) -> Definition<'x> {
    let args: Vec<_> = body
        .iter()
        .map(|tt| top_level::ingest_tree(tt, env, in_scopes))
        .collect();
    Definition::BuiltinOperation(op, args)
}
