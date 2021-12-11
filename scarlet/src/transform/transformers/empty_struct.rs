use itertools::Itertools;
use maplit::hashmap;

use crate::{
    constructs::{
        self,
        base::ConstructDefinition,
        downcast_construct,
        structt::{self, CEmptyStruct, CPopulatedStruct, SField, SFieldAndRest},
        variable::CVariable,
    },
    tfers,
    tokens::structure::Token,
    transform::{
        apply,
        basics::{ApplyContext, Transformer, TransformerResult},
        pattern::{PatCaptureStream, PatFirstOf, PatPlain, Pattern, PatternMatchSuccess},
        transformers::operators::Is,
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
        success: PatternMatchSuccess<'_, 't>,
    ) -> TransformerResult<'t> {
        let con = CEmptyStruct;
        let con_id = c.env.push_construct(Box::new(con), vec![]);
        TransformerResult(Token::Construct(con_id))
    }

    fn vomit<'x>(&self, c: &mut ApplyContext<'_, 'x>, to: &Token<'x>) -> Option<Token<'x>> {
        None
    }
}
