use maplit::hashmap;

use super::base::SpecialMember;
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

pub struct Eager;
impl SpecialMember for Eager {
    fn aliases(&self) -> &'static [&'static str] {
        &["Eager", "E"]
    }

    fn expects_paren_group(&self) -> bool {
        true
    }

    fn apply<'t>(
        &self,
        base: TokenTree<'t>,
        paren_group: Option<Vec<TokenTree<'t>>>,
    ) -> TokenTree<'t> {
        TokenTree::BuiltinRule {
            name: "eager",
            body: vec![
                base,
                TokenTree::BuiltinRule {
                    name: "values",
                    body: paren_group.unwrap(),
                },
            ],
        }
    }
}
