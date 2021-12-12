use crate::{
    tokens::structure::Token,
    transform::{
        basics::{Transformer, TransformerResult},
        pattern::{PatCaptureAny, PatPlain, Pattern, PatternMatchSuccess},
        ApplyContext,
    },
};

pub struct OnPattern;
impl Transformer for OnPattern {
    fn input_pattern(&self) -> Box<dyn Pattern> {
        Box::new((
            PatPlain("ON"),
            PatCaptureAny { key: "pattern" },
            PatCaptureAny { key: "value" },
        ))
    }

    fn apply<'t>(
        &self,
        c: &mut ApplyContext<'_, 't>,
        success: PatternMatchSuccess<'_, 't>,
    ) -> TransformerResult<'t> {
        let pattern = success.get_capture("pattern").clone();
        let pattern = c.env.push_unresolved(pattern);
        let value = success.get_capture("value").clone();
        let value = c.env.push_unresolved(value);
        TransformerResult(Token::Stream {
            label: "ON",
            contents: vec![pattern.into(), value.into()],
        })
    }

    fn vomit<'x>(&self, _c: &mut ApplyContext<'_, 'x>, _to: &Token<'x>) -> Option<Token<'x>> {
        None
    }
}

pub struct Else;
impl Transformer for Else {
    fn input_pattern(&self) -> Box<dyn Pattern> {
        Box::new((PatPlain("ELSE"), PatCaptureAny { key: "value" }))
    }

    fn apply<'t>(
        &self,
        c: &mut ApplyContext<'_, 't>,
        success: PatternMatchSuccess<'_, 't>,
    ) -> TransformerResult<'t> {
        let value = success.get_capture("value").clone();
        let value = c.env.push_unresolved(value);
        TransformerResult(Token::Stream {
            label: "ELSE",
            contents: vec![value.into()],
        })
    }

    fn vomit<'x>(&self, _c: &mut ApplyContext<'_, 'x>, _to: &Token<'x>) -> Option<Token<'x>> {
        None
    }
}
