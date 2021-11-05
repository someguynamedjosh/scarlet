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

pub struct Variable;
impl SpecialMember for Variable {
    fn aliases(&self) -> &'static [&'static str] {
        &["Variable", "Var", "V"]
    }

    fn apply<'t>(
        &self,
        base: TokenTree<'t>,
        _paren_group: Option<Vec<TokenTree<'t>>>,
    ) -> TokenTree<'t> {
        TokenTree::BuiltinRule {
            name: "variable",
            body: vec![base],
        }
    }
}
