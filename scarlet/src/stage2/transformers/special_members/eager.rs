use super::base::SpecialMember;
use crate::stage2::structure::{Environment, Token};

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
        env: &mut Environment<'t>,
        base: Token<'t>,
        paren_group: Option<Vec<Token<'t>>>,
    ) -> Token<'t> {
        Token::Stream {
            label: "eager",
            contents: vec![
                base,
                Token::Stream {
                    label: "values",
                    contents: paren_group.unwrap(),
                },
            ],
        }
    }
}
