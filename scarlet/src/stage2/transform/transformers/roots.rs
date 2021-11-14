use itertools::Itertools;
use maplit::hashmap;

use crate::{
    stage2::{
        structure::{Definition, StructField, Token, VarType},
        transform::{
            apply,
            basics::{Transformer, TransformerResult},
            pattern::{PatCaptureStream, PatPlain, Pattern, PatternMatchSuccess},
            transformers::operators::Is,
            ApplyContext,
        },
    },
    tfers,
};

pub struct SubExpression;
impl Transformer for SubExpression {
    fn pattern(&self) -> Box<dyn Pattern> {
        Box::new(PatCaptureStream {
            key: "sub_expression",
            label: "group[]",
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
}

pub struct Struct;
impl Transformer for Struct {
    fn pattern(&self) -> Box<dyn Pattern> {
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
        let extras = hashmap![200 => tfers![Is]];
        let item = c.begin_item();
        let mut c = c.with_parent_scope(Some(item));
        apply::apply_transformers(&mut c, &mut contents, &extras);
        let fields = contents
            .into_iter()
            .map(|x| match x {
                Token::Stream {
                    label: "target",
                    contents,
                } => {
                    let (name, value) = contents.into_iter().collect_tuple().unwrap();
                    let name = Some(name.unwrap_plain());
                    let value = c.push_token(value);
                    StructField { name, value }
                }
                other => {
                    let name = None;
                    let value = c.push_token(other);
                    StructField { name, value }
                }
            })
            .collect_vec();
        let def = Definition::Struct(fields);
        c.env.items[item].definition = Some(def);
        c.env.check(item);
        TransformerResult(Token::Item(item))
    }
}

pub struct Builtin;
impl Transformer for Builtin {
    fn pattern(&self) -> Box<dyn Pattern> {
        Box::new((
            PatPlain("Builtin"),
            PatCaptureStream {
                key: "args",
                label: "group()",
            },
        ))
    }

    fn apply<'t>(
        &self,
        c: &mut ApplyContext<'_, 't>,
        success: PatternMatchSuccess<'_, 't>,
    ) -> TransformerResult<'t> {
        let mut body = success.get_capture("args").unwrap_stream().clone();
        assert!(body.len() >= 1);
        let name = body.remove(0).unwrap_plain();
        apply::apply_transformers(c, &mut body, &Default::default());
        let item = match name {
            "PATTERN" => c.env.push_var(VarType::God),
            "BOOL" => c.env.push_var(VarType::Bool),
            "32U" => c.env.push_var(VarType::_32U),
            "ARRAY" => {
                let length = c.push_token(body.remove(0));
                let element_type = c.push_token(body.remove(0));
                let typee = VarType::Array {
                    length,
                    element_type,
                };
                c.env.push_var(typee)
            }
            other => todo!("Nice error, unrecognized builtin {}", other),
        };
        TransformerResult(Token::Item(item))
    }
}
