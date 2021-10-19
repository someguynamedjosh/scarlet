use std::collections::HashMap;

use super::structure::{Environment, ItemId};
use crate::{
    stage1::structure::{Module, TokenTree},
    stage2::structure::{BuiltinValue, Definition, Item, StructField, Variable},
};

struct MaybeTarget<'x> {
    target: Option<(&'x TokenTree<'x>, &'x str)>,
    value: &'x TokenTree<'x>,
}

fn maybe_target<'x>(input: &'x TokenTree<'x>) -> MaybeTarget<'x> {
    if let TokenTree::PrimitiveRule {
        name: "target",
        body,
    } = input
    {
        assert_eq!(body.len(), 2);
        let target = &body[0];
        let target_token = target.as_token().expect("TODO: Nice error");
        MaybeTarget {
            target: Some((target, target_token)),
            value: &body[1],
        }
    } else {
        MaybeTarget {
            target: None,
            value: input,
        }
    }
}

fn begin_item<'x>(src: &'x TokenTree<'x>, env: &mut Environment<'x>) -> ItemId<'x> {
    env.items.push(Item {
        original_definition: src,
        definition: None,
    })
}

fn ingest_tree<'x>(
    src: &'x TokenTree<'x>,
    env: &mut Environment<'x>,
    in_scopes: &[&HashMap<&str, ItemId<'x>>],
) -> ItemId<'x> {
    let into = begin_item(src, env);
    ingest_tree_into(src, env, into, in_scopes);
    into
}

fn ingest_tree_into<'x>(
    src: &'x TokenTree<'x>,
    env: &mut Environment<'x>,
    into: ItemId<'x>,
    in_scopes: &[&HashMap<&str, ItemId<'x>>],
) {
    let definition = match src {
        TokenTree::Token(token) => {
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
        TokenTree::PrimitiveRule {
            name: "struct",
            body,
        } => {
            let fields: Vec<_> = body.iter().map(maybe_target).collect();
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
            let new_scopes = with_extra_scope(in_scopes, &scope_map);

            for (field, id) in fields.iter().zip(ids.iter()) {
                ingest_tree_into(field.value, env, *id, &new_scopes[..]);
            }
            let mut labeled_fields = Vec::new();
            for (field, id) in fields.iter().zip(ids.iter()) {
                let name = field.target.clone().map(|x| x.1.to_owned());
                labeled_fields.push(StructField { name, value: *id });
            }
            Definition::Struct(labeled_fields)
        }
        TokenTree::PrimitiveRule {
            name: "variable",
            body,
        } => {
            if body.len() != 1 {
                todo!("Nice error");
            }
            let pattern = &body[0];
            let pattern = ingest_tree(pattern, env, in_scopes);
            let var = env.vars.push(Variable { pattern });
            Definition::Variable(var)
        }
        TokenTree::PrimitiveRule { name, .. } => todo!("{}", name),
    };
    env.items.get_mut(into).definition = Some(definition);
}

fn ingest_module<'x>(
    src: &'x Module,
    env: &mut Environment<'x>,
    into: ItemId<'x>,
    in_scopes: &[&HashMap<&str, ItemId<'x>>],
) {
    let mut children = Vec::new();
    for (name, module) in &src.children {
        assert_eq!(module.self_content.len(), 1);
        let src = &module.self_content[0];
        children.push((&name[..], module, begin_item(src, env)));
    }

    let scope_map: HashMap<_, _> = children.iter().map(|(name, _, id)| (*name, *id)).collect();
    let new_scopes = with_extra_scope(in_scopes, &scope_map);

    assert_eq!(src.self_content.len(), 1);
    ingest_tree_into(&src.self_content[0], env, into, in_scopes);

    for (_, src, id) in children {
        ingest_module(src, env, id, &new_scopes[..]);
    }
}

fn with_extra_scope<'b, 'c, 'x>(
    in_scopes: &[&'b HashMap<&'c str, ItemId<'x>>],
    scope_to_add: &'b HashMap<&'c str, ItemId<'x>>,
) -> Vec<&'b HashMap<&'c str, ItemId<'x>>> {
    in_scopes
        .iter()
        .copied()
        .chain(std::iter::once(scope_to_add))
        .collect()
}

pub fn ingest<'x>(src: &'x Module) -> Environment<'x> {
    assert_eq!(src.self_content.len(), 1);
    let mut env = Environment::new();
    let into = begin_item(&src.self_content[0], &mut env);
    ingest_module(src, &mut env, into, &[]);
    env
}
