use crate::{
    constructs::builtin_value::CBuiltinValue,
    tokens::structure::Token,
    transform::{
        basics::{Transformer, TransformerResult},
        pattern::{PatCaptureAny, PatPlain, Pattern, PatternMatchSuccess},
        ApplyContext,
    },
};

pub struct Unique;
impl Transformer for Unique {
    fn input_pattern(&self) -> Box<dyn Pattern> {
        Box::new(PatPlain("UNIQUE"))
    }

    fn apply<'t>(
        &self,
        c: &mut ApplyContext<'_, 't>,
        _success: PatternMatchSuccess<'_, 't>,
    ) -> TransformerResult<'t> {
        let id = c.env.push_unique();
        let con = c.env.push_construct(Box::new(CBuiltinValue::Unique(id)));
        TransformerResult(Token::Construct(con))
    }

    fn vomit<'x>(&self, _c: &mut ApplyContext<'_, 'x>, _to: &Token<'x>) -> Option<Token<'x>> {
        None
    }
}