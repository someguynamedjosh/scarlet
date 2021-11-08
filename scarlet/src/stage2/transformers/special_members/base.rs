use crate::stage2::{
    structure::Token,
    transformers::{
        apply,
        basics::{Extras, Transformer, TransformerResult},
        helpers,
    },
};

pub trait SpecialMember {
    fn aliases(&self) -> &'static [&'static str];
    fn expects_paren_group(&self) -> bool {
        false
    }
    fn paren_group_transformers<'t>(&self) -> Extras<'t> {
        Default::default()
    }
    fn apply<'t>(&self, base: Token<'t>, paren_group: Option<Vec<Token<'t>>>) -> Token<'t>;
}

impl<M: SpecialMember> Transformer for M {
    fn should_be_applied_at(&self, to: &[Token], at: usize) -> bool {
        if at < 1 {
            return false;
        }
        if &to[at] != &Token::Plain(".") {
            false
        } else {
            for alias in self.aliases() {
                if &to[at + 1] == &Token::Plain(alias) {
                    return true;
                }
            }
            false
        }
    }

    fn apply<'t>(&self, to: &Vec<Token<'t>>, at: usize) -> TransformerResult<'t> {
        let mut end = at + 1;
        let base = to[at - 1].clone();
        let paren_group = if self.expects_paren_group() {
            end += 1;
            let mut paren_group = helpers::expect_paren_group(&to[end]).clone();
            let extras = self.paren_group_transformers();
            apply::apply_transformers(&mut paren_group, &extras);
            Some(paren_group)
        } else {
            None
        };
        let replace_with_tree = <Self as SpecialMember>::apply(&self, base, paren_group);
        TransformerResult {
            replace_range: at - 1..=end,
            with: replace_with_tree,
        }
    }
}
