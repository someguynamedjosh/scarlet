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

fn ingest_tree<'x>(src: &'x TokenTree<'x>, env: &mut Environment<'x>) -> ItemId<'x> {
    let into = begin_item(src, env);
    ingest_tree_into(src, env, into);
    into
}

fn ingest_tree_into<'x>(src: &'x TokenTree<'x>, env: &mut Environment<'x>, into: ItemId<'x>) {
    let definition = match src {
        TokenTree::Token(token) => {
            if let Ok(num) = token.parse() {
                Definition::BuiltinValue(BuiltinValue::U32(num))
            } else {
                todo!("ident")
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
            for (field, id) in fields.iter().zip(ids.iter()) {
                ingest_tree_into(field.value, env, *id);
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
            let pattern = ingest_tree(pattern, env);
            let var = env.vars.push(Variable { pattern });
            Definition::Variable(var)
        }
        TokenTree::PrimitiveRule { name, .. } => todo!("{}", name),
    };
    env.items.get_mut(into).definition = Some(definition);
}

fn ingest_module<'x>(src: &'x Module, env: &mut Environment<'x>, into: ItemId<'x>) {
    for (name, module) in &src.children {
        todo!()
    }
    assert_eq!(src.self_content.len(), 1);
    ingest_tree_into(&src.self_content[0], env, into);
}

pub fn ingest<'x>(src: &'x Module) -> Environment<'x> {
    assert_eq!(src.self_content.len(), 1);
    let mut env = Environment::new();
    let into = begin_item(&src.self_content[0], &mut env);
    ingest_module(src, &mut env, into);
    env
}
