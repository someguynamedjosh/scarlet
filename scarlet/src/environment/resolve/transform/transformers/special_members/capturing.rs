use crate::{
    environment::resolve::transform::{
        transformers::special_members::base::SpecialMember, ApplyContext,
    },
    tokens::structure::Token,
};

pub struct Capturing;
impl SpecialMember for Capturing {
    fn aliases(&self) -> &'static [&'static str] {
        &["CAPTURING", "CAP", "C"]
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
            if token == "ALL".into() {
                all = true
            } else {
                vals.push(Token::Construct(c.push_unresolved(token)))
            }
        }
        if all {
            vals = vec!["ALL".into()];
        }
        Token::Stream {
            label: "CAPTURING",
            contents: [vec![base], vals].concat(),
        }
    }
}
