use crate::{
    tokens::structure::Token,
    transform::{transformers::special_members::base::SpecialMember, ApplyContext},
};

pub struct AsLanguageItem;
impl SpecialMember for AsLanguageItem {
    fn aliases(&self) -> &'static [&'static str] {
        &["AS_LANGUAGE_ITEM", "AS_LANG_ITEM", "ALI"]
    }

    fn expects_bracket_group(&self) -> bool {
        true
    }

    fn apply<'t>(
        &self,
        c: &mut ApplyContext<'_, 't>,
        base: Token<'t>,
        bracket_group: Option<Vec<Token<'t>>>,
    ) -> Token<'t> {
        let id = c.push_unresolved(base);
        let base = Token::Construct(id);
        // TODO: Nice errors.
        let bracket_group = bracket_group.unwrap();
        let name = bracket_group[0].unwrap_plain();
        c.env.define_builtin_item(name, id);
        base
    }

    fn vomit<'x>(&self, _c: &mut ApplyContext<'_, 'x>, _to: &Token<'x>) -> Option<Token<'x>> {
        None
    }
}
