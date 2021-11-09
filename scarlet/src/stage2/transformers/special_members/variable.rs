use super::base::SpecialMember;
use crate::stage2::structure::{Environment, Token};

pub struct Variable;
impl SpecialMember for Variable {
    fn aliases(&self) -> &'static [&'static str] {
        &["Variable", "Var", "V"]
    }

    fn apply<'t>(
        &self,
        env: &mut Environment<'t>,
        base: Token<'t>,
        _paren_group: Option<Vec<Token<'t>>>,
    ) -> Token<'t> {
        Token::Stream {
            label: "variable",
            contents: vec![base],
        }
    }
}
