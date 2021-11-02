use std::collections::HashMap;

use super::top_level::IngestionContext;
use crate::{
    stage1::structure::{Token, TokenTree},
    stage2::structure::{Environment, Item, ItemId},
};

pub struct MaybeTarget<'x> {
    pub target: Option<(&'x TokenTree<'x>, &'x str)>,
    pub value: &'x TokenTree<'x>,
}

pub fn maybe_target<'x>(input: &'x TokenTree<'x>) -> MaybeTarget<'x> {
    if let TokenTree::BuiltinRule {
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

impl<'e, 'x> IngestionContext<'e, 'x> {
    pub(super) fn begin_item(&mut self, src: &'x TokenTree<'x>) -> ItemId<'x> {
        let mut total_scope = HashMap::new();
        for scope in self.in_scopes {
            for (ident, value) in *scope {
                total_scope.insert(*ident, *value);
            }
        }
        self.env.items.push(Item {
            dependencies: None,
            original_definition: src,
            definition: None,
            scope: total_scope,
            cached_reduction: None,
            shown_from: Vec::new(),
        })
    }
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
