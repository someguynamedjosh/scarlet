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

pub struct WithoutCapturing;
impl SpecialMember for WithoutCapturing {
    fn aliases(&self) -> &'static [&'static str] {
        &["WITHOUT_CAPTURING", "WO_CAP", "WC"]
    }

    fn expects_bracket_group(&self) -> bool {
        true
    }

    fn apply<'t>(
        &self,
        c: &mut ApplyContext<'_, 't>,
        base: Token<'t>,
        bracket_group: Option<Vec<Token<'t>>>,
    ) -> Token<'t> {
        let base = Token::Construct(c.push_unresolved(base));
        let mut vals = Vec::new();
        let mut all = false;
        for token in bracket_group.unwrap() {
            if let Token::Plain("ALL") = token {
                all = true
            } else {
                vals.push(Token::Construct(c.push_unresolved(token)))
            }
        }
        if all {
            vals = vec![Token::Plain("ALL")];
        }
        Token::Stream {
            label: "WITHOUT_CAPTURING",
            contents: [vec![base], vals].concat(),
        }
    }
}
