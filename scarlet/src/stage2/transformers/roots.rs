use itertools::Itertools;
use maplit::hashmap;

use super::basics::ApplyContext;
use crate::{
    stage2::{
        structure::{Definition, Environment, StructField, Token, VarType},
        transformers::{
            apply,
            basics::{Transformer, TransformerResult},
            helpers,
            operators::Is,
        },
    },
    tfers,
};

pub struct SubExpression;
impl Transformer for SubExpression {
    fn should_be_applied_at<'t>(&self, c: &mut ApplyContext<'_, 't>, at: usize) -> bool {
        if let Token::Stream {
            label: "group[]", ..
        } = &c.to[at]
        {
            true
        } else {
            false
        }
    }

    fn apply<'t>(&self, c: &mut ApplyContext<'_, 't>, at: usize) -> TransformerResult<'t> {
        if let Token::Stream { contents: body, .. } = &c.to[at] {
            let mut body = body.clone();
            apply::apply_transformers(&mut c.with_target(&mut body), &Default::default());
            assert_eq!(body.len(), 1);
            TransformerResult {
                replace_range: at..=at,
                with: body.into_iter().next().unwrap(),
            }
        } else {
            unreachable!("Checked in should_be_applied_at")
        }
    }
}

pub struct Struct;
impl Transformer for Struct {
    fn should_be_applied_at<'t>(&self, c: &mut ApplyContext<'_, 't>, at: usize) -> bool {
        if let Token::Stream { label: name, .. } = &c.to[at] {
            *name == "group{}"
        } else {
            false
        }
    }

    fn apply<'t>(&self, c: &mut ApplyContext<'_, 't>, at: usize) -> TransformerResult<'t> {
        if let Token::Stream { contents, .. } = &c.to[at] {
            let mut contents = contents.clone();
            let extras = hashmap![200 => tfers![Is]];
            let item = c.env.begin_item();
            apply::apply_transformers(&mut c.with_target(&mut contents), &extras);
            let fields = contents
                .into_iter()
                .map(|x| match x {
                    Token::Stream {
                        label: "target",
                        contents,
                    } => {
                        let (name, value) = contents.into_iter().collect_tuple().unwrap();
                        let name = Some(name.unwrap_plain());
                        let value = c.env.push_def(Definition::Unresolved(value));
                        StructField { name, value }
                    }
                    other => {
                        let name = None;
                        let value = c.env.push_def(Definition::Unresolved(other));
                        StructField { name, value }
                    }
                })
                .collect_vec();
            let def = Definition::Struct(fields);
            c.env.items[item].definition = Some(def);
            TransformerResult {
                replace_range: at..=at,
                with: Token::Item(item),
            }
        } else {
            unreachable!("Checked in should_be_applied_at")
        }
    }
}

pub struct Builtin;
impl Transformer for Builtin {
    fn should_be_applied_at<'t>(&self, c: &mut ApplyContext<'_, 't>, at: usize) -> bool {
        &c.to[at] == &Token::Plain("Builtin")
    }

    fn apply<'t>(&self, c: &mut ApplyContext<'_, 't>, at: usize) -> TransformerResult<'t> {
        let mut body = helpers::expect_paren_group(&c.to[at + 1]).clone();
        assert!(body.len() >= 1);
        let name = body.remove(0).unwrap_plain();
        apply::apply_transformers(&mut c.with_target(&mut body), &Default::default());
        let item = match name {
            "PATTERN" => c.env.push_var(VarType::God),
            "BOOL" => c.env.push_var(VarType::Bool),
            "32U" => c.env.push_var(VarType::_32U),
            other => todo!("Nice error, unrecognized builtin {}", other),
        };
        TransformerResult {
            replace_range: at..=at + 1,
            with: Token::Item(item),
        }
    }
}
