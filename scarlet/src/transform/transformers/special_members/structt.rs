use crate::{
    constructs::structt::{AtomicStructMember, CAtomicStructMember},
    tokens::structure::Token,
    transform::{transformers::special_members::base::SpecialMember, ApplyContext},
};

pub struct StructLabel;
impl SpecialMember for StructLabel {
    fn aliases(&self) -> &'static [&'static str] {
        &["LABEL"]
    }

    fn apply<'t>(
        &self,
        c: &mut ApplyContext<'_, 't>,
        base: Token<'t>,
        _paren_group: Option<Vec<Token<'t>>>,
    ) -> Token<'t> {
        let base = c.push_unresolved(base);
        Token::Construct(c.env.push_construct(
            Box::new(CAtomicStructMember(base, AtomicStructMember::Label)),
            vec![base],
        ))
    }

    fn vomit<'x>(&self, _c: &mut ApplyContext<'_, 'x>, _to: &Token<'x>) -> Option<Token<'x>> {
        None
    }
}

pub struct StructValue;
impl SpecialMember for StructValue {
    fn aliases(&self) -> &'static [&'static str] {
        &["VALUE"]
    }

    fn apply<'t>(
        &self,
        c: &mut ApplyContext<'_, 't>,
        base: Token<'t>,
        _paren_group: Option<Vec<Token<'t>>>,
    ) -> Token<'t> {
        let base = c.push_unresolved(base);
        Token::Construct(c.env.push_construct(
            Box::new(CAtomicStructMember(base, AtomicStructMember::Value)),
            vec![base],
        ))
    }

    fn vomit<'x>(&self, _c: &mut ApplyContext<'_, 'x>, _to: &Token<'x>) -> Option<Token<'x>> {
        None
    }
}

pub struct StructRest;
impl SpecialMember for StructRest {
    fn aliases(&self) -> &'static [&'static str] {
        &["REST"]
    }

    fn apply<'t>(
        &self,
        c: &mut ApplyContext<'_, 't>,
        base: Token<'t>,
        _paren_group: Option<Vec<Token<'t>>>,
    ) -> Token<'t> {
        let base = c.push_unresolved(base);
        Token::Construct(c.env.push_construct(
            Box::new(CAtomicStructMember(base, AtomicStructMember::Rest)),
            vec![base],
        ))
    }

    fn vomit<'x>(&self, _c: &mut ApplyContext<'_, 'x>, _to: &Token<'x>) -> Option<Token<'x>> {
        None
    }
}
