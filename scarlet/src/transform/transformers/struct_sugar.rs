use maplit::hashmap;

use crate::{
    constructs::{
        downcast_construct,
        structt::{CPopulatedStruct, SField, SFieldAndRest},
    },
    tfers,
    tokens::structure::Token,
    transform::{
        apply,
        basics::{ApplyContext, Transformer, TransformerResult},
        pattern::{PatCaptureStream, PatPlain, Pattern, PatternMatchSuccess},
        transformers::operators::Is,
    },
};

pub struct StructSugar;
impl Transformer for StructSugar {
    fn input_pattern(&self) -> Box<dyn Pattern> {
        Box::new(PatCaptureStream {
            key: "fields",
            label: "group{}",
        })
    }

    fn apply<'t>(
        &self,
        c: &mut ApplyContext<'_, 't>,
        success: PatternMatchSuccess<'_, 't>,
    ) -> TransformerResult<'t> {
        let mut contents = success.get_capture("fields").unwrap_stream().clone();
        let extras = hashmap![200 => tfers![ Is ]];
        apply::apply_transformers(c, &mut contents, &extras);

        let mut fields = vec![];
        for field_def in contents {
            match field_def {
                Token::Stream {
                    label: "target",
                    mut contents,
                } => {
                    assert_eq!(contents.len(), 2);
                    let label = contents[0].unwrap_plain().to_owned();
                    fields.push((label, contents.remove(1)));
                }
                _ => {
                    fields.push((String::new(), field_def));
                }
            }
        }

        let mut result = vec![c.env.get_builtin_item("void").into()];
        for field in fields.into_iter().rev() {
            let body = [vec![field.0.into(), field.1], result].concat();
            result = vec![
                format!("POPULATED_STRUCT").into(),
                Token::Stream {
                    label: "group[]",
                    contents: body,
                },
            ]
        }

        apply::apply_transformers(c, &mut result, &Default::default());
        assert_eq!(result.len(), 1);
        TransformerResult(result.remove(0))
    }

    fn vomit<'x>(&self, c: &mut ApplyContext<'_, 'x>, to: &Token<'x>) -> Option<Token<'x>> {
        None
    }
}
