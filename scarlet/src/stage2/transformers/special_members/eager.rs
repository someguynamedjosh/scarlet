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
        let vals = paren_group
            .unwrap()
            .into_iter()
            .map(|x| c.push_token(x))
            .collect_vec();
        let def = Definition::SetEager {
            base: c.push_token(base),
            vals,
            eager: true,
        };
        let item = c.push_def(def);
        Token::Item(item)
    }
}
