use itertools::Itertools;

use crate::{
    constructs::{
        self,
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
                key: "invariants",
                label: "group[]",
            },
        ))
    }

    fn apply<'t>(
        &self,
        c: &mut ApplyContext<'_, 't>,
        success: PatternMatchSuccess<'_, 't>,
    ) -> TransformerResult<'t> {
        let mut invariants = success.get_capture("invariants").unwrap_stream().clone();
        apply::apply_transformers(c, &mut invariants, &Default::default());

        let invariants = invariants
            .into_iter()
            .map(|x| c.env.push_unresolved(x))
            .collect_vec();
        let id = c.env.variables.push(constructs::variable::Variable);

        CVariable::new(c.env, id, invariants, false).into()
    }

    fn vomit<'x>(&self, _c: &mut ApplyContext<'_, 'x>, _to: &Token<'x>) -> Option<Token<'x>> {
        None
    }
}
