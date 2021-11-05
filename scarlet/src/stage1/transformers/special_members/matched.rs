use maplit::hashmap;

use super::base::SpecialMember;
use crate::{
    stage1::{
        structure::TokenTree,
        transformers::{
            basics::{Extras, Transformer},
            statements::{Else, OnPattern},
        },
    },
    tfers,
};

pub struct Matched;
impl SpecialMember for Matched {
    fn aliases(&self) -> &'static [&'static str] {
        &["Matched", "M"]
    }

    fn expects_paren_group(&self) -> bool {
        true
    }

    fn paren_group_transformers<'t>(&self) -> Extras<'t> {
        hashmap![172 => tfers![OnPattern, Else]]
    }

    fn apply<'t>(
        &self,
        base: TokenTree<'t>,
        paren_group: Option<Vec<TokenTree<'t>>>,
    ) -> TokenTree<'t> {
        TokenTree::BuiltinRule {
            name: "match",
            body: vec![
                base,
                TokenTree::BuiltinRule {
                    name: "patterns",
                    body: paren_group.unwrap(),
                },
            ],
        }
    }
}
