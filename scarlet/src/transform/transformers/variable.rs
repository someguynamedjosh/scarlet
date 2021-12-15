use crate::{
    constructs::variable::SVariableInvariants,
    tokens::structure::Token,
    transform::{
        apply,
        basics::{ApplyContext, Transformer, TransformerResult},
        pattern::{PatCaptureStream, PatFirstOf, PatPlain, Pattern, PatternMatchSuccess},
    },
};

pub struct Variable;
impl Transformer for Variable {
    fn input_pattern(&self) -> Box<dyn Pattern> {
        Box::new((
            PatFirstOf(vec![
                Box::new(PatPlain("VARIABLE")),
                Box::new(PatPlain("VAR")),
                Box::new(PatPlain("V")),
            ]),
            PatCaptureStream {
                key: "arguments",
                label: "group[]",
            },
        ))
    }

    fn apply<'t>(
        &self,
        c: &mut ApplyContext<'_, 't>,
        success: PatternMatchSuccess<'_, 't>,
    ) -> TransformerResult<'t> {
        let mut arguments = success.get_capture("arguments").unwrap_stream().clone();
        apply::apply_transformers(c, &mut arguments, &Default::default());

        let mut depends_on = Vec::new();
        let mut invariants = Vec::new();
        let mut mode = 0;
        for token in arguments {
            if token == "DEPENDS_ON".into() {
                mode = 1;
                continue;
            }
            match mode {
                0 => invariants.push(c.env.push_unresolved(token)),
                1 => depends_on.push(c.env.push_unresolved(token)),
                _ => unreachable!(),
            }
        }

        let con_id = c.env.push_unresolved(Token::Stream {
            label: "VARIABLE",
            contents: vec![
                Token::Stream {
                    label: "INVARIANTS",
                    contents: invariants.clone().into_iter().map(Into::into).collect(),
                },
                Token::Stream {
                    label: "DEPENDS_ON",
                    contents: depends_on.into_iter().map(Into::into).collect(),
                },
            ],
        });

        for invariant in invariants {
            c.env.set_scope(invariant, &SVariableInvariants(con_id));
        }

        con_id.into()
    }

    fn vomit<'x>(&self, _c: &mut ApplyContext<'_, 'x>, _to: &Token<'x>) -> Option<Token<'x>> {
        None
    }
}
