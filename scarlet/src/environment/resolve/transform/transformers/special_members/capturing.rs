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

pub struct Shy;
impl SpecialMember for Shy {
    fn aliases(&self) -> &'static [&'static str] {
        &["CAPTURING"]
    }

    fn expects_bracket_group(&self) -> bool {
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
                vals.push(c.push_unresolved(token))
            }
        }
        let def = Definition::SetEager {
            base: c.push_unresolved(base),
            vals,
            all,
            eager: false,
        };
        let con = c.push_construct(def);
        Token::Construct(con)
    }
}
