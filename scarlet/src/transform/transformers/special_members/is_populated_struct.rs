use crate::{
    constructs::{is_populated_struct::CIsPopulatedStruct, shown::CShown},
    tokens::structure::Token,
    transform::{transformers::special_members::base::SpecialMember, ApplyContext},
};

pub struct IsPopulatedStruct;
impl SpecialMember for IsPopulatedStruct {
    fn aliases(&self) -> &'static [&'static str] {
        &["IS_POPULATED_STRUCT"]
    }

    fn apply<'t>(
        &self,
        c: &mut ApplyContext<'_, 't>,
        base: Token<'t>,
        _paren_group: Option<Vec<Token<'t>>>,
    ) -> Token<'t> {
        let base = c.push_unresolved(base);
        let con = Box::new(CIsPopulatedStruct(base));
        Token::Construct(c.env.push_construct(con, vec![base]))
    }

    fn vomit<'x>(&self, _c: &mut ApplyContext<'_, 'x>, _to: &Token<'x>) -> Option<Token<'x>> {
        None
    }
}
