use itertools::Itertools;
use maplit::hashmap;

use crate::{
    constructs::{
        base::ConstructDefinition,
        downcast_construct,
        structt::{CStruct, StructField},
        variable::VarType,
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

pub struct Struct;
impl Transformer for Struct {
    fn input_pattern(&self) -> Box<dyn Pattern> {
        Box::new(PatCaptureStream {
            key: "fields",
            label: "group[]",
        })
    }

    fn apply<'t>(
        &self,
        c: &mut ApplyContext<'_, 't>,
        success: PatternMatchSuccess<'_, 't>,
    ) -> TransformerResult<'t> {
        let mut contents = success.get_capture("fields").unwrap_stream().clone();
        let extras = hashmap![200 => tfers![Is]];
        let con = c.push_placeholder();
        let mut c = c.with_parent_scope(Some(con));
        apply::apply_transformers(&mut c, &mut contents, &extras);
        let fields = contents
            .into_iter()
            .map(|x| match x {
                Token::Stream {
                    label: "target",
                    contents,
                } => {
                    let (name, value) = contents.into_iter().collect_tuple().unwrap();
                    let name = Some(name.unwrap_plain().to_owned());
                    let value = c.push_unresolved(value);
                    StructField { name, value }
                }
                other => {
                    let name = None;
                    let value = c.push_unresolved(other);
                    StructField { name, value }
                }
            })
            .collect_vec();
        let def = Box::new(CStruct(fields));
        c.env.constructs[con].definition = ConstructDefinition::Resolved(def);
        c.env.check(con);
        TransformerResult(Token::Construct(con))
    }

    fn vomit<'x>(&self, c: &mut ApplyContext<'_, 'x>, to: &Token<'x>) -> Option<Token<'x>> {
        if let &Token::Construct(con_id) = to {
            if let Some(structt) = downcast_construct::<CStruct>(&**c.env.get_construct(con_id)) {
                let mut contents = Vec::new();
                for field in &structt.0 {
                    let value: Token = field.value.into();
                    if let Some(name) = field.name.clone() {
                        contents.push(Token::Stream {
                            label: "target",
                            contents: vec![name.into(), value],
                        })
                    } else {
                        contents.push(value);
                    }
                }
                return Some(Token::Stream {
                    label: "group[]",
                    contents,
                });
            }
        }
        None
    }
}
