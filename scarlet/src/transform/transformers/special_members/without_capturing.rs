use crate::{
    tokens::structure::Token,
    transform::{transformers::special_members::base::SpecialMember, ApplyContext},
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
            label: "WITHOUT_CAPTURING",
            contents: [vec![base], vals].concat(),
        }
    }

    fn vomit<'x>(&self, c: &mut ApplyContext<'_, 'x>, to: &Token<'x>) -> Option<Vec<Token<'x>>> {
        None
    }
}
