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
pub struct SubExpression;
impl Transformer for SubExpression {
    fn input_pattern(&self) -> Box<dyn Pattern> {
        Box::new(PatCaptureStream {
            key: "sub_expression",
            label: "group()",
        })
    }

    fn apply<'t>(
        &self,
        c: &mut ApplyContext<'_, 't>,
        success: PatternMatchSuccess<'_, 't>,
    ) -> TransformerResult<'t> {
        let mut body = success
            .get_capture("sub_expression")
            .unwrap_stream()
            .clone();
        apply::apply_transformers(c, &mut body, &Default::default());
        assert_eq!(body.len(), 1);
        TransformerResult(body.into_iter().next().unwrap())
    }

    fn vomit<'x>(&self, _c: &mut ApplyContext<'_, 'x>, _to: &Token<'x>) -> Option<Token<'x>> {
        None
    }
}
