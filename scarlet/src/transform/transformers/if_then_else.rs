use crate::{
    constructs::{
        downcast_construct,
        if_then_else::CIfThenElse,
        structt::{CPopulatedStruct, SField, SFieldAndRest},
    },
    scope::SPlain,
    tokens::structure::Token,
    transform::{
        apply,
        basics::{ApplyContext, Transformer, TransformerResult},
        pattern::{PatCaptureStream, PatPlain, Pattern, PatternMatchSuccess},
    },
};

pub struct IfThenElse;
impl Transformer for IfThenElse {
    fn input_pattern(&self) -> Box<dyn Pattern> {
        Box::new((
            PatPlain("IF_THEN_ELSE"),
            PatCaptureStream {
                key: "args",
                label: "group[]",
            },
        ))
    }

    fn apply<'t>(
        &self,
        c: &mut ApplyContext<'_, 't>,
        success: PatternMatchSuccess<'_, 't>,
    ) -> TransformerResult<'t> {
        let mut contents = success.get_capture("args").unwrap_stream().clone();
        apply::apply_transformers(c, &mut contents, &Default::default());
        assert_eq!(contents.len(), 3);

        let condition = c.env.push_unresolved(contents[0].clone());
        let then = c.env.push_unresolved(contents[1].clone());
        let elsee = c.env.push_unresolved(contents[2].clone());

        CIfThenElse::new(c.env, condition, then, elsee).into()
    }

    fn vomit<'x>(&self, c: &mut ApplyContext<'_, 'x>, to: &Token<'x>) -> Option<Token<'x>> {
        None
    }
}
