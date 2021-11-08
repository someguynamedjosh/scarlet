use super::base::SpecialMember;
use crate::stage2::structure::Token;

pub struct Shown;
impl SpecialMember for Shown {
    fn aliases(&self) -> &'static [&'static str] {
        &["Shown", "S"]
    }

    fn apply<'t>(&self, base: Token<'t>, _paren_group: Option<Vec<Token<'t>>>) -> Token<'t> {
        Token::Stream {
            label: "shown",
            contents: vec![base],
        }
    }
}
