use maplit::hashmap;

use super::basics::ApplyContext;
use crate::{
    stage2::{
        structure::{Environment, Token},
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
    fn should_be_applied_at<'t>(&self, c: &mut ApplyContext<'_, 't>, at: usize) -> bool {
        if at == 0 {
            false
        } else if let Token::Stream {
            label: "group()", ..
        } = &c.to[at]
        {
            true
        } else {
            false
        }
    }

    fn apply<'t>(&self, c: &mut ApplyContext<'_, 't>, at: usize) -> TransformerResult<'t> {
        let base = c.to[at - 1].clone();
        if let Token::Stream { contents: body, .. } = &c.to[at] {
            let mut substitutions = body.clone();
            let extras = hashmap![200 => tfers![Is]];
            apply::apply_transformers(&mut c.with_target(&mut substitutions), &extras);
            TransformerResult {
                replace_range: at - 1..=at,
                with: Token::Stream {
                    label: "substitute",
                    contents: [vec![base], substitutions].concat(),
                },
            }
        } else {
            unreachable!("Checked in should_be_applied_at")
        }
    }
}
