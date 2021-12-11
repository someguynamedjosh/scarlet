use crate::{
    constructs::shown::CShown,
    tokens::structure::Token,
    transform::{transformers::special_members::base::SpecialMember, ApplyContext},
};

pub struct Shown;
impl SpecialMember for Shown {
    fn aliases(&self) -> &'static [&'static str] {
        &["SHOWING", "SHOW", "S"]
    }

    fn apply<'t>(
        &self,
        c: &mut ApplyContext<'_, 't>,
        base: Token<'t>,
        _paren_group: Option<Vec<Token<'t>>>,
    ) -> Token<'t> {
        let base = c.push_unresolved(base);
        Token::Construct(c.env.push_construct(Box::new(CShown(base)), vec![base]))
    }

    fn vomit<'x>(&self, _c: &mut ApplyContext<'_, 'x>, _to: &Token<'x>) -> Option<Token<'x>> {
        None
    }
}
