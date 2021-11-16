use crate::{
    environment::resolve::transform::{
        basics::ApplyContext, transformers::special_members::base::SpecialMember,
    },
    tokens::structure::Token,
};

pub struct Eager;
impl SpecialMember for Eager {
    fn aliases(&self) -> &'static [&'static str] {
        &["Eager", "E"]
    }

    fn expects_bracket_group(&self) -> bool {
        true
    }

    fn apply<'t>(
        &self,
        c: &mut ApplyContext<'_, 't>,
        base: Token<'t>,
        paren_group: Option<Vec<Token<'t>>>,
    ) -> Token<'t> {
        let mut vals = Vec::new();
        let mut all = false;
        for token in paren_group.unwrap() {
            if let Token::Plain("All") = token {
                all = true
            } else {
                vals.push(c.push_unresolved(token))
            }
        }
        let def = Definition::SetEager {
            base: c.push_unresolved(base),
            vals,
            all,
            eager: true,
        };
        let con = c.push_construct(def);
        Token::Construct(con)
    }
}
