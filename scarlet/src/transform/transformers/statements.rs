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
        let pattern = Token::Construct(c.push_unresolved(success.get_capture("pattern").clone()));
        let value = Token::Construct(c.push_unresolved(success.get_capture("value").clone()));
        TransformerResult(Token::Stream {
            label: "ON",
            contents: vec![pattern, value],
        })
    }

    fn vomit<'x>(&self, c: &mut ApplyContext<'_, 'x>, to: &Token<'x>) -> Option<Vec<Token<'x>>> {
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
        let value = Token::Construct(c.push_unresolved(success.get_capture("value").clone()));
        TransformerResult(Token::Stream {
            label: "ELSE",
            contents: vec![value],
        })
    }

    fn vomit<'x>(&self, c: &mut ApplyContext<'_, 'x>, to: &Token<'x>) -> Option<Vec<Token<'x>>> {
        None
    }
}
