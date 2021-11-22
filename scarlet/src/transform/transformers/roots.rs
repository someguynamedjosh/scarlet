use itertools::Itertools;
use maplit::hashmap;

use crate::{
    constructs::{
        base::ConstructDefinition,
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

    fn vomit<'x>(&self, _c: &mut ApplyContext<'_, 'x>, _to: &Token<'x>) -> Option<Vec<Token<'x>>> {
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

    fn vomit<'x>(&self, _c: &mut ApplyContext<'_, 'x>, _to: &Token<'x>) -> Option<Vec<Token<'x>>> {
        None
    }
}

pub struct Builtin;
impl Transformer for Builtin {
    fn input_pattern(&self) -> Box<dyn Pattern> {
        Box::new((
            PatPlain("BUILTIN"),
            PatCaptureStream {
                key: "args",
                label: "group{}",
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
        let name = body.remove(0);
        let name = name.unwrap_plain();
        apply::apply_transformers(c, &mut body, &Default::default());
        let con = match name {
            "Anything" => c.push_var(VarType::Anything, true),
            "Boolean" => c.push_var(VarType::Bool, true),
            "32BitUnsignedInteger" => c.push_var(VarType::_32U, true),
            "array" => {
                let length = c.push_unresolved(body.remove(0));
                let eltype = c.push_unresolved(body.remove(0));
                let typee = VarType::Array { length, eltype };
                c.push_var(typee, false)
            }
            other => todo!("Nice error, unrecognized builtin {}", other),
        };
        TransformerResult(Token::Construct(con))
    }

    fn vomit<'x>(&self, _c: &mut ApplyContext<'_, 'x>, _to: &Token<'x>) -> Option<Vec<Token<'x>>> {
        None
    }
}
