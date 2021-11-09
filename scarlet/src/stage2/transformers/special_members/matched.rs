use itertools::Itertools;
use maplit::hashmap;

use super::base::SpecialMember;
use crate::{
    stage2::{
        structure::{Condition, Definition, Environment, Token},
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
        let base = env.push_token(base);
        let mut conditions = Vec::new();
        let mut else_value = None;
        for token in paren_group.unwrap() {
            match token {
                Token::Stream {
                    label: "on",
                    contents,
                } => {
                    let (pattern, value) = contents.into_iter().collect_tuple().unwrap();
                    let (pattern, value) = (env.push_token(pattern), env.push_token(value));
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
                    else_value = Some(env.push_token(value));
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
        let item = env.push_def(def);
        Token::Item(item)
    }
}
