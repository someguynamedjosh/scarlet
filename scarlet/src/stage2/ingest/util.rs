use std::collections::HashMap;

use crate::{
    stage1::structure::TokenTree,
    stage2::structure::{Environment, Item, ItemId},
};

pub struct MaybeTarget<'x> {
    pub target: Option<(&'x TokenTree<'x>, &'x str)>,
    pub value: &'x TokenTree<'x>,
}

pub fn maybe_target<'x>(input: &'x TokenTree<'x>) -> MaybeTarget<'x> {
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

pub fn begin_item<'x>(src: &'x TokenTree<'x>, env: &mut Environment<'x>) -> ItemId<'x> {
    env.items.push(Item {
        original_definition: src,
        definition: None,
    })
}

pub fn with_extra_scope<'b, 'c, 'x>(
    in_scopes: &[&'b HashMap<&'c str, ItemId<'x>>],
    scope_to_add: &'b HashMap<&'c str, ItemId<'x>>,
) -> Vec<&'b HashMap<&'c str, ItemId<'x>>> {
    in_scopes
        .iter()
        .copied()
        .chain(std::iter::once(scope_to_add))
        .collect()
}
