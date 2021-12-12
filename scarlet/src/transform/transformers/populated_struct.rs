use crate::{
    constructs::{
        downcast_construct,
        structt::{CPopulatedStruct, SField, SFieldAndRest},
    },
    tokens::structure::Token,
    transform::{
        apply,
        basics::{ApplyContext, Transformer, TransformerResult},
        pattern::{PatCaptureStream, PatPlain, Pattern, PatternMatchSuccess},
    },
};

pub struct PopulatedStruct;
impl Transformer for PopulatedStruct {
    fn input_pattern(&self) -> Box<dyn Pattern> {
        Box::new((
            PatPlain("POPULATED_STRUCT"),
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
        let label = if contents.len() == 3 {
            contents.remove(0).unwrap_plain().to_owned()
        } else {
            String::new()
        };
        assert_eq!(contents.len(), 2);

        let value = c.env.push_unresolved(contents[0].clone());
        let rest = c.env.push_unresolved(contents[1].clone());

        CPopulatedStruct::new(c.env, label, value, rest).into()
    }

    fn vomit<'x>(&self, c: &mut ApplyContext<'_, 'x>, to: &Token<'x>) -> Option<Token<'x>> {
        if let &Token::Construct(con_id) = to {
            if let Some(structt) =
                downcast_construct::<CPopulatedStruct>(&**c.env.get_construct_definition(con_id))
            {
                let contents = vec![
                    structt.get_label().to_owned().into(),
                    structt.get_value().into(),
                    structt.get_rest().into(),
                ];
                return Some(Token::Stream {
                    label: "group[]",
                    contents,
                });
            }
        }
        None
    }
}
