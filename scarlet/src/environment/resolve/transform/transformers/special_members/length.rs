use crate::{
    constructs::length::CLength,
    environment::resolve::transform::{
        transformers::special_members::base::SpecialMember, ApplyContext,
    },
    tokens::structure::Token,
};

pub struct Length;
impl SpecialMember for Length {
    fn aliases(&self) -> &'static [&'static str] {
        &["LENGTH", "L"]
    }

    fn apply<'t>(
        &self,
        c: &mut ApplyContext<'_, 't>,
        base: Token<'t>,
        _paren_group: Option<Vec<Token<'t>>>,
    ) -> Token<'t> {
        let base = c.push_unresolved(base);
        Token::Construct(c.push_construct(Box::new(CLength(base))))
    }
}
