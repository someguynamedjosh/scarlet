use itertools::Itertools;
use maplit::hashmap;

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
    fn should_be_applied_at(&self, to: &[Token], at: usize) -> bool {
        if let Token::Stream {
            label: "group[]", ..
        } = &to[at]
        {
            true
        } else {
            false
        }
    }

    fn apply<'t>(
        &self,
        env: &mut Environment<'t>,
        to: &Vec<Token<'t>>,
        at: usize,
    ) -> TransformerResult<'t> {
        if let Token::Stream { contents: body, .. } = &to[at] {
            let mut body = body.clone();
            apply::apply_transformers(env, &mut body, &Default::default());
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
    fn should_be_applied_at(&self, to: &[Token], at: usize) -> bool {
        if let Token::Stream { label: name, .. } = &to[at] {
            *name == "group{}"
        } else {
            false
        }
    }

    fn apply<'t>(
        &self,
        env: &mut Environment<'t>,
        to: &Vec<Token<'t>>,
        at: usize,
    ) -> TransformerResult<'t> {
        if let Token::Stream { contents, .. } = &to[at] {
            let mut contents = contents.clone();
            let extras = hashmap![200 => tfers![Is]];
            apply::apply_transformers(env, &mut contents, &extras);
            let fields = contents
                .into_iter()
                .map(|x| match x {
                    Token::Stream {
                        label: "target",
                        contents,
                    } => {
                        let (name, value) = contents.into_iter().collect_tuple().unwrap();
                        let name = Some(name.unwrap_plain());
                        let value = env.push_def(Definition::Resolvable(value));
                        StructField { name, value }
                    }
                    other => {
                        let name = None;
                        let value = env.push_def(Definition::Resolvable(other));
                        StructField { name, value }
                    }
                })
                .collect_vec();
            let def = Definition::Struct(fields);
            let item = env.push_def(def);
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
    fn should_be_applied_at(&self, to: &[Token], at: usize) -> bool {
        &to[at] == &Token::Plain("Builtin")
    }

    fn apply<'t>(
        &self,
        env: &mut Environment<'t>,
        to: &Vec<Token<'t>>,
        at: usize,
    ) -> TransformerResult<'t> {
        let mut body = helpers::expect_paren_group(&to[at + 1]).clone();
        assert!(body.len() >= 1);
        let name = body.remove(0).unwrap_plain();
        apply::apply_transformers(env, &mut body, &Default::default());
        let item = match name {
            "PATTERN" => env.push_var(VarType::God),
            "BOOL" => env.push_var(VarType::Bool),
            "32U" => env.push_var(VarType::_32U),
            other => todo!("Nice error, unrecognized builtin {}", other),
        };
        TransformerResult {
            replace_range: at..=at + 1,
            with: Token::Item(item),
        }
    }
}
