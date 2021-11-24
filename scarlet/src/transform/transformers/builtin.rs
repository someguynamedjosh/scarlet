use crate::{
    constructs::{
        builtin_operation::{BuiltinOperation, CBuiltinOperation},
        length::CLength,
    },
    tokens::structure::Token,
    transform::{
        apply,
        basics::{ApplyContext, Transformer, TransformerResult},
        pattern::{PatCaptureStream, PatPlain, Pattern, PatternMatchSuccess},
    },
};

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
            "length" => {
                let of = c.push_unresolved(body.remove(0));
                let con = CLength(of);
                c.push_construct(Box::new(con))
            }
            other => todo!("Nice error, unrecognized builtin {}", other),
        };
        TransformerResult(Token::Construct(con))
    }

    fn vomit<'x>(&self, _c: &mut ApplyContext<'_, 'x>, _to: &Token<'x>) -> Option<Token<'x>> {
        None
    }
}
