use super::base::SpecialMember;
use crate::stage2::{
    structure::{Definition, Token},
    transform::ApplyContext,
};

pub struct Shy;
impl SpecialMember for Shy {
    fn aliases(&self) -> &'static [&'static str] {
        &["Shy"]
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
            eager: false,
        };
        let item = c.push_def(def);
        Token::Item(item)
    }
}
