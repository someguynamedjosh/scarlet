use std::{borrow::Cow, collections::HashMap, ops::RangeInclusive};

use maplit::hashmap;

use crate::{
    stage1::{
        structure::TokenTree,
        transformers::{
            apply,
            basics::{
                Extras, OwnedOrBorrowed, Precedence, SomeTransformer, Transformer,
                TransformerResult,
            },
            helpers,
        },
    },
    tfers,
};

pub struct OnPattern;
impl Transformer for OnPattern {
    fn should_be_applied_at(&self, to: &[TokenTree], at: usize) -> bool {
        &to[at] == &TokenTree::Token("on")
    }

    fn apply<'t>(&self, to: &Vec<TokenTree<'t>>, at: usize) -> TransformerResult<'t> {
        let pattern = to[at + 1].clone();
        let pattern = TokenTree::BuiltinRule {
            name: "pattern",
            body: vec![pattern],
        };
        let value = to[at + 2].clone();
        TransformerResult {
            replace_range: at..=at + 2,
            with: TokenTree::BuiltinRule {
                name: "on",
                body: vec![pattern, value],
            },
        }
    }
}

pub struct Else;
impl Transformer for Else {
    fn should_be_applied_at(&self, to: &[TokenTree], at: usize) -> bool {
        &to[at] == &TokenTree::Token("else")
    }

    fn apply<'t>(&self, to: &Vec<TokenTree<'t>>, at: usize) -> TransformerResult<'t> {
        let value = to[at + 1].clone();
        TransformerResult {
            replace_range: at..=at + 1,
            with: TokenTree::BuiltinRule {
                name: "else",
                body: vec![value],
            },
        }
    }
}
