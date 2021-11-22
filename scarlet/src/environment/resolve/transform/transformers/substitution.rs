use maplit::hashmap;

use crate::{
    environment::resolve::transform::{
        apply,
        basics::{Transformer, TransformerResult},
        pattern::{PatCaptureAny, PatCaptureStream, Pattern, PatternMatchSuccess},
        transformers::operators::Is,
        ApplyContext,
    },
    tfers,
    tokens::structure::Token,
};

pub struct Substitution;
impl Transformer for Substitution {
    fn input_pattern(&self) -> Box<dyn Pattern> {
        Box::new((
            PatCaptureAny { key: "base" },
            PatCaptureStream {
                key: "subs",
                label: "group{}",
            },
        ))
    }

    fn apply<'t>(
        &self,
        c: &mut ApplyContext<'_, 't>,
        success: PatternMatchSuccess<'_, 't>,
    ) -> TransformerResult<'t> {
        let base = success.get_capture("base").clone();
        let mut substitutions = success.get_capture("subs").unwrap_stream().clone();
        let extras = hashmap![200 => tfers![Is]];
        apply::apply_transformers(c, &mut substitutions, &extras);
        TransformerResult(Token::Stream {
            label: "substitute",
            contents: [vec![base], substitutions].concat(),
        })
    }
}
