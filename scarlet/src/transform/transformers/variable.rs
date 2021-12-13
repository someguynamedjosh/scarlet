use itertools::Itertools;

use crate::{
    constructs::{
        self, as_variable,
        structt::{self},
        variable::{CVariable, SVariableInvariants},
    },
    scope::{SPlaceholder, SPlain},
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
                1 => {
                    let construct = if let Token::Construct(con) = token {
                        con
                    } else {
                        todo!("Nice error, dependency must be a variable.");
                    };
                    let def = c.env.get_construct_definition(construct);
                    let var = if let Some(var) = as_variable(&**def) {
                        var.clone()
                    } else {
                        todo!("Nice error, dependency must be a variable.");
                    };
                    depends_on.push(var)
                }
                _ => unreachable!(),
            }
        }

        let id = c.env.variables.push(constructs::variable::Variable);

        CVariable::new(c.env, id, invariants, false, depends_on).into()
    }

    fn vomit<'x>(&self, _c: &mut ApplyContext<'_, 'x>, _to: &Token<'x>) -> Option<Token<'x>> {
        None
    }
}
