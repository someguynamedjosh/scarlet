use crate::{
    constructs::variable::VarType,
    environment::resolve::transform::{
        transformers::special_members::base::SpecialMember, ApplyContext,
    },
    tokens::structure::Token,
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
        let pattern = c.push_unresolved(base);
        let var_con = c.push_var(VarType::Just(pattern), false);
        Token::Construct(var_con)
    }
}
