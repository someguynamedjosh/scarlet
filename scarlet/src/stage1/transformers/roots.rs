use maplit::hashmap;

use crate::{
    stage1::transformers::{
        apply,
        basics::{Transformer, TransformerResult},
        helpers,
        operators::Is,
    },
    stage2::structure::Token,
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

    fn apply<'t>(&self, to: &Vec<Token<'t>>, at: usize) -> TransformerResult<'t> {
        if let Token::Stream { contents: body, .. } = &to[at] {
            let mut body = body.clone();
            apply::apply_transformers(&mut body, &Default::default());
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

    fn apply<'t>(&self, to: &Vec<Token<'t>>, at: usize) -> TransformerResult<'t> {
        if let Token::Stream { contents: body, .. } = &to[at] {
            let mut body = body.clone();
            let extras = hashmap![200 => tfers![Is]];
            apply::apply_transformers(&mut body, &extras);
            let name = "struct";
            TransformerResult {
                replace_range: at..=at,
                with: Token::Stream {
                    label: name,
                    contents: body,
                },
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

    fn apply<'t>(&self, to: &Vec<Token<'t>>, at: usize) -> TransformerResult<'t> {
        let mut body = helpers::expect_paren_group(&to[at + 1]).clone();
        assert!(body.len() >= 1);
        let name = body.remove(0).unwrap_plain();
        apply::apply_transformers(&mut body, &Default::default());
        TransformerResult {
            replace_range: at..=at + 1,
            with: Token::Stream {
                label: name,
                contents: body,
            },
        }
    }
}
