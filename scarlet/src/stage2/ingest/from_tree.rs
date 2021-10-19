use std::collections::HashMap;

use crate::{
    stage1::structure::TokenTree,
    stage2::{
        ingest::{top_level, util},
        structure::{BuiltinValue, Definition, Environment, Item, ItemId, StructField, Variable},
    },
};

pub fn definition_from_tree<'x>(
    src: &'x TokenTree<'x>,
    env: &mut Environment<'x>,
    in_scopes: &[&HashMap<&str, ItemId<'x>>],
) -> Definition<'x> {
    match src {
        TokenTree::Token(token) => token_def(token, in_scopes),
        TokenTree::PrimitiveRule {
            name: "struct",
            body,
        } => struct_def(body, in_scopes, env),
        TokenTree::PrimitiveRule {
            name: "variable",
            body,
        } => variable_def(body, env, in_scopes),
        TokenTree::PrimitiveRule { name, .. } => todo!("{}", name),
    }
}

fn token_def<'x>(token: &&str, in_scopes: &[&HashMap<&str, ItemId<'x>>]) -> Definition<'x> {
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

fn struct_def<'x>(
    body: &'x Vec<TokenTree<'x>>,
    in_scopes: &[&HashMap<&str, ItemId<'x>>],
    env: &mut Environment<'x>,
) -> Definition<'x> {
    let fields: Vec<_> = body.iter().map(util::maybe_target).collect();
    let ids: Vec<_> = fields
        .iter()
        .map(|target| Item {
            original_definition: target.value,
            definition: None,
        })
        .map(|item| env.items.push(item))
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

fn variable_def<'x>(
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
