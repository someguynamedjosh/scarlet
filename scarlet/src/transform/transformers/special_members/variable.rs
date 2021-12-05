use crate::{
    tokens::structure::Token,
    transform::{transformers::special_members::base::SpecialMember, ApplyContext},
};

pub struct Variable;
impl SpecialMember for Variable {
    fn aliases(&self) -> &'static [&'static str] {
        &["VARIABLE", "VAR", "V"]
    }

    fn apply<'t>(
        &self,
        c: &mut ApplyContext<'_, 't>,
        base: Token<'t>,
        _paren_group: Option<Vec<Token<'t>>>,
    ) -> Token<'t> {
        let invariant = c.push_unresolved(base);
        let var_con = c.push_var(invariant, false);
        Token::Construct(var_con)
    }

    fn vomit<'x>(&self, c: &mut ApplyContext<'_, 'x>, to: &Token<'x>) -> Option<Token<'x>> {
        None
    }
}
