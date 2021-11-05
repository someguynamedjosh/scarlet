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

pub struct Shown;
impl SpecialMember for Shown {
    fn aliases(&self) -> &'static [&'static str] {
        &["Shown", "S"]
    }

    fn apply<'t>(
        &self,
        base: TokenTree<'t>,
        _paren_group: Option<Vec<TokenTree<'t>>>,
    ) -> TokenTree<'t> {
        TokenTree::BuiltinRule {
            name: "shown",
            body: vec![base],
        }
    }
}
