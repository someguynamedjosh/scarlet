use maplit::hashmap;

use super::base::SpecialMember;
use crate::{
    stage2::{
        structure::{Environment, Token},
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
        env: &mut Environment<'t>,
        base: Token<'t>,
        paren_group: Option<Vec<Token<'t>>>,
    ) -> Token<'t> {
        Token::Stream {
            label: "matched",
            contents: vec![
                base,
                Token::Stream {
                    label: "patterns",
                    contents: paren_group.unwrap(),
                },
            ],
        }
    }
}
