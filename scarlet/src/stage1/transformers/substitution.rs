use maplit::hashmap;

use crate::{
    stage1::{
        structure::TokenTree,
        transformers::{
            apply,
            basics::{Transformer, TransformerResult},
            operators::Is,
        },
    },
    tfers,
};

pub struct Substitution;
impl Transformer for Substitution {
    fn should_be_applied_at(&self, to: &[TokenTree], at: usize) -> bool {
        if at == 0 {
            false
        } else if let TokenTree::BuiltinRule {
            name: "group()", ..
        } = &to[at]
        {
            true
        } else {
            false
        }
    }

    fn apply<'t>(&self, to: &Vec<TokenTree<'t>>, at: usize) -> TransformerResult<'t> {
        let base = to[at - 1].clone();
        if let TokenTree::BuiltinRule { body, .. } = &to[at] {
            let mut substitutions = body.clone();
            let extras = hashmap![200 => tfers![Is]];
            apply::apply_transformers(&mut substitutions, &extras);
            let substitutions = TokenTree::BuiltinRule {
                name: "substitutions",
                body: substitutions,
            };
            TransformerResult {
                replace_range: at - 1..=at,
                with: TokenTree::BuiltinRule {
                    name: "substitute",
                    body: vec![base, substitutions],
                },
            }
        } else {
            unreachable!("Checked in should_be_applied_at")
        }
    }
}
