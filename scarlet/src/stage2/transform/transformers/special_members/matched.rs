use itertools::Itertools;
use maplit::hashmap;

use super::base::SpecialMember;
use crate::{
    stage2::{
        structure::{Condition, Definition, Token},
        transform::{
            basics::Extras,
            transformers::statements::{Else, OnPattern},
            ApplyContext,
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
        c: &mut ApplyContext<'_, 't>,
        base: Token<'t>,
        paren_group: Option<Vec<Token<'t>>>,
    ) -> Token<'t> {
        let base = c.push_token(base);
        let mut conditions = Vec::new();
        let mut else_value = None;
        for token in paren_group.unwrap() {
            match token {
                Token::Stream {
                    label: "on",
                    contents,
                } => {
                    let (pattern, value) = contents.into_iter().collect_tuple().unwrap();
                    let (pattern, value) = (c.push_token(pattern), c.push_token(value));
                    conditions.push(Condition { pattern, value })
                }
                Token::Stream {
                    label: "else",
                    contents,
                } => {
                    if else_value.is_some() {
                        todo!("Nice error.")
                    }
                    let (value,) = contents.into_iter().collect_tuple().unwrap();
                    else_value = Some(c.push_token(value));
                }
                _ => todo!("Nice error"),
            }
        }
        let else_value = else_value.unwrap();
        let def = Definition::Match {
            base,
            conditions,
            else_value,
        };
        let item = c.push_con(def);
        Token::Item(item)
    }
}
