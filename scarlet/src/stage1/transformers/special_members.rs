use maplit::hashmap;

use crate::{
    stage1::{
        structure::TokenTree,
        transformers::{
            apply,
            basics::{Extras, Transformer, TransformerResult},
            helpers,
            statements::{Else, OnPattern},
        },
    },
    tfers,
};

pub struct Matched;
impl Transformer for Matched {
    fn should_be_applied_at(&self, to: &[TokenTree], at: usize) -> bool {
        if at < 1 {
            return false;
        }
        &to[at] == &TokenTree::Token(".") && &to[at + 1] == &TokenTree::Token("Matched")
    }

    fn apply<'t>(&self, to: &Vec<TokenTree<'t>>, at: usize) -> TransformerResult<'t> {
        let base = to[at - 1].clone();
        let mut patterns = helpers::expect_paren_group(&to[at + 2]).clone();
        let extras: Extras = hashmap![172 => tfers![OnPattern, Else]];
        apply::apply_transformers(&mut patterns, &extras);
        let patterns = TokenTree::BuiltinRule {
            name: "patterns",
            body: patterns,
        };
        TransformerResult {
            replace_range: at - 1..=at + 2,
            with: TokenTree::BuiltinRule {
                name: "match",
                body: vec![base, patterns],
            },
        }
    }
}

pub struct Shown;
impl Transformer for Shown {
    fn should_be_applied_at(&self, to: &[TokenTree], at: usize) -> bool {
        &to[at] == &TokenTree::Token(".")
            && (&to[at + 1] == &TokenTree::Token("Shown") || &to[at + 1] == &TokenTree::Token("S"))
    }

    fn apply<'t>(&self, to: &Vec<TokenTree<'t>>, at: usize) -> TransformerResult<'t> {
        let value = to[at - 1].clone();
        TransformerResult {
            replace_range: at - 1..=at + 1,
            with: TokenTree::BuiltinRule {
                name: "show",
                body: vec![value],
            },
        }
    }
}

pub struct Eager;
impl Transformer for Eager {
    fn should_be_applied_at(&self, to: &[TokenTree], at: usize) -> bool {
        &to[at] == &TokenTree::Token(".")
            && (&to[at + 1] == &TokenTree::Token("Eager") || &to[at + 1] == &TokenTree::Token("E"))
    }

    fn apply<'t>(&self, to: &Vec<TokenTree<'t>>, at: usize) -> TransformerResult<'t> {
        let mut vals = helpers::expect_paren_group(&to[at + 2]).clone();
        apply::apply_transformers(&mut vals, &Default::default());
        let vals = TokenTree::BuiltinRule {
            name: "vals",
            body: vals,
        };
        let base = to[at - 1].clone();
        TransformerResult {
            replace_range: at - 1..=at + 2,
            with: TokenTree::BuiltinRule {
                name: "eager",
                body: vec![vals, base],
            },
        }
    }
}

pub struct Variable;
impl Transformer for Variable {
    fn should_be_applied_at(&self, to: &[TokenTree], at: usize) -> bool {
        if at < 1 {
            return false;
        }
        &to[at] == &TokenTree::Token(".")
            && (&to[at + 1] == &TokenTree::Token("Variable")
                || &to[at + 1] == &TokenTree::Token("Var")
                || &to[at + 1] == &TokenTree::Token("V"))
    }

    fn apply<'t>(&self, to: &Vec<TokenTree<'t>>, at: usize) -> TransformerResult<'t> {
        let pattern = to[at - 1].clone();
        TransformerResult {
            replace_range: at - 1..=at + 1,
            with: TokenTree::BuiltinRule {
                name: "variable",
                body: vec![pattern],
            },
        }
    }
}
