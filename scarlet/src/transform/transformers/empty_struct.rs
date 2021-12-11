use crate::{
    constructs::structt::CEmptyStruct,
    tokens::structure::Token,
    transform::{
        basics::{ApplyContext, Transformer, TransformerResult},
        pattern::{PatPlain, Pattern, PatternMatchSuccess},
    },
};

pub struct EmptyStruct;
impl Transformer for EmptyStruct {
    fn input_pattern(&self) -> Box<dyn Pattern> {
        Box::new(PatPlain("EMPTY_STRUCT"))
    }

    fn apply<'t>(
        &self,
        c: &mut ApplyContext<'_, 't>,
        _success: PatternMatchSuccess<'_, 't>,
    ) -> TransformerResult<'t> {
        let con = CEmptyStruct;
        let con_id = c.env.push_construct(Box::new(con), vec![]);
        TransformerResult(Token::Construct(con_id))
    }

    fn vomit<'x>(&self, _c: &mut ApplyContext<'_, 'x>, _to: &Token<'x>) -> Option<Token<'x>> {
        None
    }
}
