use itertools::Itertools;

use super::base::SpecialMember;
use crate::stage2::{
    structure::{Definition, Environment, Token},
    transformers::ApplyContext,
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
        c: &mut ApplyContext<'_, 't>,
        base: Token<'t>,
        paren_group: Option<Vec<Token<'t>>>,
    ) -> Token<'t> {
        let mut vals = Vec::new();
        let mut all = false;
        for token in paren_group.unwrap() {
            if let Token::Plain("All") = token {
                all = true
            } else {
                vals.push(c.push_token(token))
            }
        }
        let def = Definition::SetEager {
            base: c.push_token(base),
            vals,
            all,
            eager: true,
        };
        let item = c.push_def(def);
        Token::Item(item)
    }
}
