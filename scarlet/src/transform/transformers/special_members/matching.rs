use itertools::Itertools;
use maplit::hashmap;

use crate::{
    constructs::matchh::{CMatch, Condition},
    tfers,
    tokens::structure::Token,
    transform::{
        basics::Extras,
        transformers::{
            special_members::base::SpecialMember,
            statements::{Else, OnPattern},
        },
        ApplyContext,
    },
};

pub struct Matching;
impl SpecialMember for Matching {
    fn aliases(&self) -> &'static [&'static str] {
        &["MATCHING", "M"]
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
        bracket_group: Option<Vec<Token<'t>>>,
    ) -> Token<'t> {
        let base = c.push_unresolved(base);
        let mut conditions = Vec::new();
        let mut else_value = None;
        println!("{:?}", bracket_group);
        for token in bracket_group.unwrap() {
            match token {
                Token::Stream {
                    label: "ON",
                    contents,
                } => {
                    let (pattern, value) = contents.into_iter().collect_tuple().unwrap();
                    let (pattern, value) = (c.push_unresolved(pattern), c.push_unresolved(value));
                    conditions.push(Condition { pattern, value })
                }
                Token::Stream {
                    label: "ELSE",
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
        let def = CMatch {
            base,
            conditions,
            else_value,
        };
        let con = c.push_construct(Box::new(def));
        Token::Construct(con)
    }

    fn vomit<'x>(&self, _c: &mut ApplyContext<'_, 'x>, _to: &Token<'x>) -> Option<Vec<Token<'x>>> {
        None
    }
}
