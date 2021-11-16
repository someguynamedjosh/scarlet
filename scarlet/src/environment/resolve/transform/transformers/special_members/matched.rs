use itertools::Itertools;
use maplit::hashmap;

use crate::{
    environment::resolve::transform::{
        basics::Extras,
        transformers::{
            special_members::base::SpecialMember,
            statements::{Else, OnPattern},
        },
        ApplyContext,
    },
    tfers,
    tokens::structure::Token,
};

pub struct Matched;
impl SpecialMember for Matched {
    fn aliases(&self) -> &'static [&'static str] {
        &["Matched", "M"]
    }

    fn expects_bracket_group(&self) -> bool {
        true
    }

    fn bracket_group_transformers<'t>(&self) -> Extras<'t> {
        hashmap![172 => tfers![OnPattern, Else]]
    }

    fn apply<'t>(
        &self,
        c: &mut ApplyContext<'_, 't>,
        base: Token<'t>,
        paren_group: Option<Vec<Token<'t>>>,
    ) -> Token<'t> {
        let base = c.push_unresolved(base);
        let mut conditions = Vec::new();
        let mut else_value = None;
        for token in paren_group.unwrap() {
            match token {
                Token::Stream {
                    label: "on",
                    contents,
                } => {
                    let (pattern, value) = contents.into_iter().collect_tuple().unwrap();
                    let (pattern, value) = (c.push_unresolved(pattern), c.push_unresolved(value));
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
                    else_value = Some(c.push_unresolved(value));
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
        let con = c.push_construct(def);
        Token::Construct(con)
    }
}
