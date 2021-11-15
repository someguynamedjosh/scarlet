use crate::stage2::{
    structure::Token,
    transform::{
        basics::{Transformer, TransformerResult},
        pattern::{PatCaptureAny, PatPlain, Pattern, PatternMatchSuccess},
        ApplyContext,
    },
};

pub struct OnPattern;
impl Transformer for OnPattern {
    fn pattern(&self) -> Box<dyn Pattern> {
        Box::new((
            PatPlain("on"),
            PatCaptureAny { key: "pattern" },
            PatCaptureAny { key: "value" },
        ))
    }

    fn apply<'t>(
        &self,
        c: &mut ApplyContext<'_, 't>,
        success: PatternMatchSuccess<'_, 't>,
    ) -> TransformerResult<'t> {
        let pattern = Token::Item(c.push_token(success.get_capture("pattern").clone()));
        let value = Token::Item(c.push_token(success.get_capture("value").clone()));
        TransformerResult(Token::Stream {
            label: "on",
            contents: vec![pattern, value],
        })
    }
}

pub struct Else;
impl Transformer for Else {
    fn pattern(&self) -> Box<dyn Pattern> {
        Box::new((PatPlain("else"), PatCaptureAny { key: "value" }))
    }

    fn apply<'t>(
        &self,
        c: &mut ApplyContext<'_, 't>,
        success: PatternMatchSuccess<'_, 't>,
    ) -> TransformerResult<'t> {
        let value = Token::Item(c.push_token(success.get_capture("value").clone()));
        TransformerResult(Token::Stream {
            label: "else",
            contents: vec![value],
        })
    }
}
