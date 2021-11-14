use crate::stage2::{
    structure::Token,
    transform::{
        apply,
        basics::{ApplyContext, Extras, Transformer, TransformerResult},
        pattern::{
            PatCaptureAny, PatCaptureStream, PatFirstOf, PatPlain, Pattern, PatternMatchSuccess,
        },
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
        c: &mut ApplyContext<'_, 't>,
        base: Token<'t>,
        paren_group: Option<Vec<Token<'t>>>,
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
        if self.expects_paren_group() {
            Box::new((
                base,
                PatCaptureStream {
                    key: "args",
                    label: "group()",
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
        let paren_group = if self.expects_paren_group() {
            let mut paren_group = success.get_capture("args").unwrap_stream().clone();
            let extras = self.paren_group_transformers();
            apply::apply_transformers(c, &mut paren_group, &extras);
            Some(paren_group)
        } else {
            None
        };
        let replace_with_tree = <Self as SpecialMember>::apply(&self, c, base, paren_group);
        TransformerResult(replace_with_tree)
    }
}
