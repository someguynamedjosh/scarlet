use super::base::SpecialMember;
use crate::stage2::structure::{Definition, Environment, Token};

pub struct Shown;
impl SpecialMember for Shown {
    fn aliases(&self) -> &'static [&'static str] {
        &["Shown", "S"]
    }

    fn apply<'t>(
        &self,
        env: &mut Environment<'t>,
        base: Token<'t>,
        _paren_group: Option<Vec<Token<'t>>>,
    ) -> Token<'t> {
        let base = env.push_def(Definition::Resolvable(base));
        env.items[base].shown_from.push(base);
        Token::Item(base)
    }
}
