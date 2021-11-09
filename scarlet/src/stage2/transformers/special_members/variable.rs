use super::base::SpecialMember;
use crate::stage2::{
    structure::{Environment, Token, VarType},
    transformers::ApplyContext,
};

pub struct Variable;
impl SpecialMember for Variable {
    fn aliases(&self) -> &'static [&'static str] {
        &["Variable", "Var", "V"]
    }

    fn apply<'t>(
        &self,
        c: &mut ApplyContext<'_, 't>,
        base: Token<'t>,
        _paren_group: Option<Vec<Token<'t>>>,
    ) -> Token<'t> {
        let pattern = c.push_token(base);
        let var_item = c.env.push_var(VarType::Just(pattern));
        Token::Item(var_item)
    }
}
