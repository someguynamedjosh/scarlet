use maplit::hashmap;

use crate::{
    stage2::{
        structure::Token,
        transformers::{
            apply,
            basics::{Transformer, TransformerResult},
            operators::Is,
        },
    },
    tfers,
};

pub struct Substitution;
impl Transformer for Substitution {
    fn should_be_applied_at(&self, to: &[Token], at: usize) -> bool {
        if at == 0 {
            false
        } else if let Token::Stream {
            label: "group()", ..
        } = &to[at]
        {
            true
        } else {
            false
        }
    }

    fn apply<'t>(&self, to: &Vec<Token<'t>>, at: usize) -> TransformerResult<'t> {
        let base = to[at - 1].clone();
        if let Token::Stream { contents: body, .. } = &to[at] {
            let mut substitutions = body.clone();
            let extras = hashmap![200 => tfers![Is]];
            apply::apply_transformers(&mut substitutions, &extras);
            let substitutions = Token::Stream {
                label: "substitutions",
                contents: substitutions,
            };
            TransformerResult {
                replace_range: at - 1..=at,
                with: Token::Stream {
                    label: "substitute",
                    contents: vec![base, substitutions],
                },
            }
        } else {
            unreachable!("Checked in should_be_applied_at")
        }
    }
}
