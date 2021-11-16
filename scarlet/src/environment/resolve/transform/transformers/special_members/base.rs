use crate::{
    environment::resolve::transform::{
        apply,
        basics::{ApplyContext, Extras, Transformer, TransformerResult},
        pattern::{
            PatCaptureAny, PatCaptureStream, PatFirstOf, PatPlain, Pattern, PatternMatchSuccess,
        },
    },
    tokens::structure::Token,
};

pub trait SpecialMember {
    fn aliases(&self) -> &'static [&'static str];
    fn expects_bracket_group(&self) -> bool {
        false
    }
    fn bracket_group_transformers<'t>(&self) -> Extras<'t> {
        Default::default()
    }
    fn apply<'t>(
        &self,
        c: &mut ApplyContext<'_, 't>,
        base: Token<'t>,
        bracket_group: Option<Vec<Token<'t>>>,
    ) -> Token<'t>;
}

impl<M: SpecialMember> Transformer for M {
    fn pattern(&self) -> Box<dyn Pattern> {
        let base = (
            PatCaptureAny { key: "base" },
            PatPlain("."),
            PatFirstOf(
                self.aliases()
                    .iter()
                    .map(|alias| Box::new(PatPlain(*alias)) as Box<dyn Pattern>)
                    .collect(),
            ),
        );
        if self.expects_bracket_group() {
            Box::new((
                base,
                PatCaptureStream {
                    key: "args",
                    label: "group{}",
                },
            ))
        } else {
            Box::new(base)
        }
    }

    fn apply<'t>(
        &self,
        c: &mut ApplyContext<'_, 't>,
        success: PatternMatchSuccess<'_, 't>,
    ) -> TransformerResult<'t> {
        let base = success.get_capture("base").clone();
        let paren_group = if self.expects_bracket_group() {
            let mut paren_group = success.get_capture("args").unwrap_stream().clone();
            let extras = self.bracket_group_transformers();
            apply::apply_transformers(c, &mut paren_group, &extras);
            Some(paren_group)
        } else {
            None
        };
        let replace_with_tree = <Self as SpecialMember>::apply(&self, c, base, paren_group);
        TransformerResult(replace_with_tree)
    }
}
