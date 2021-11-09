use super::base::SpecialMember;
use crate::stage2::structure::{Environment, Token, VarType};

pub struct Variable;
impl SpecialMember for Variable {
    fn aliases(&self) -> &'static [&'static str] {
        &["Variable", "Var", "V"]
    }

    fn apply<'t>(
        &self,
        env: &mut Environment<'t>,
        base: Token<'t>,
        _paren_group: Option<Vec<Token<'t>>>,
    ) -> Token<'t> {
        let pattern = env.push_token(base);
        let var_item = env.push_var(VarType::Just(pattern));
        Token::Item(var_item)
    }
}
