mod match_def;
mod others;
mod struct_def;
mod substitute_def;

use std::collections::HashMap;

use crate::{
    stage1::structure::TokenTree,
    stage2::structure::{BuiltinValue, Definition, Environment, ItemId},
};

pub fn definition_from_tree<'x>(
    src: &'x TokenTree<'x>,
    env: &mut Environment<'x>,
    in_scopes: &[&HashMap<&str, ItemId<'x>>],
) -> Definition<'x> {
    match src {
        TokenTree::Token(token) => others::token_def(token, in_scopes),
        TokenTree::PrimitiveRule {
            name: "match",
            body,
        } => match_def::ingest(body, env, in_scopes),
        TokenTree::PrimitiveRule {
            name: "member",
            body,
        } => others::member_def(body, env, in_scopes),
        TokenTree::PrimitiveRule {
            name: "struct",
            body,
        } => struct_def::ingest(body, in_scopes, env),
        TokenTree::PrimitiveRule {
            name: "substitute",
            body,
        } => substitute_def::ingest(body, env, in_scopes),
        TokenTree::PrimitiveRule {
            name: "variable",
            body,
        } => others::variable_def(body, env, in_scopes),
        TokenTree::PrimitiveRule {
            name: "ANY_PATTERN",
            ..
        } => Definition::BuiltinValue(BuiltinValue::GodPattern),
        TokenTree::PrimitiveRule { name: "32U", .. } => {
            Definition::BuiltinValue(BuiltinValue::U32Pattern)
        }
        TokenTree::PrimitiveRule { name, .. } => todo!("{}", name),
    }
}
