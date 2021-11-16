use crate::{
    constructs::shown::CShown,
    environment::resolve::transform::{
        transformers::special_members::base::SpecialMember, ApplyContext,
    },
    tokens::structure::Token,
};

pub struct Shown;
impl SpecialMember for Shown {
    fn aliases(&self) -> &'static [&'static str] {
        &["SHOWN", "S"]
    }

    fn apply<'t>(
        &self,
        c: &mut ApplyContext<'_, 't>,
        base: Token<'t>,
        _paren_group: Option<Vec<Token<'t>>>,
    ) -> Token<'t> {
        let base = c.push_unresolved(base);
        Token::Construct(c.push_construct(Box::new(CShown(base))))
    }
}
