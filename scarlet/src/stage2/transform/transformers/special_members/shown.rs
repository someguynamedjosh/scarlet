use super::base::SpecialMember;
use crate::stage2::{
    structure::{Definition, Environment, Token},
    transform::ApplyContext,
};

pub struct Shown;
impl SpecialMember for Shown {
    fn aliases(&self) -> &'static [&'static str] {
        &["Shown", "S"]
    }

    fn apply<'t>(
        &self,
        c: &mut ApplyContext<'_, 't>,
        base: Token<'t>,
        _paren_group: Option<Vec<Token<'t>>>,
    ) -> Token<'t> {
        let base = c.push_def(Definition::Unresolved(base));
        c.env.items[base].shown_from.push(base);
        Token::Item(base)
    }
}
