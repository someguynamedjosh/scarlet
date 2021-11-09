use crate::stage2::{
    structure::{Environment, Token},
    transformers::{
        apply,
        basics::{ApplyContext, Extras, Transformer, TransformerResult},
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
    fn apply<'t>(
        &self,
        env: &mut Environment<'t>,
        base: Token<'t>,
        paren_group: Option<Vec<Token<'t>>>,
    ) -> Token<'t>;
}

impl<M: SpecialMember> Transformer for M {
    fn should_be_applied_at<'t>(&self, c: &mut ApplyContext<'_, 't>, at: usize) -> bool {
        if at < 1 {
            return false;
        }
        if &c.to[at] != &Token::Plain(".") {
            false
        } else {
            for alias in self.aliases() {
                if &c.to[at + 1] == &Token::Plain(alias) {
                    return true;
                }
            }
            false
        }
    }

    fn apply<'t>(&self, c: &mut ApplyContext<'_, 't>, at: usize) -> TransformerResult<'t> {
        let mut end = at + 1;
        let base = c.to[at - 1].clone();
        let paren_group = if self.expects_paren_group() {
            end += 1;
            let mut paren_group = helpers::expect_paren_group(&c.to[end]).clone();
            let extras = self.paren_group_transformers();
            apply::apply_transformers(&mut c.with_target(&mut paren_group), &extras);
            Some(paren_group)
        } else {
            None
        };
        let replace_with_tree = <Self as SpecialMember>::apply(&self, c.env, base, paren_group);
        TransformerResult {
            replace_range: at - 1..=end,
            with: replace_with_tree,
        }
    }
}
