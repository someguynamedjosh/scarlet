use itertools::Itertools;

use crate::{
    constructs::{
        self,
        structt::{self},
        variable::{CVariable, SVariableInvariants},
    },
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
            .map(|x| c.push_unresolved(x))
            .collect_vec();
        let id = c.env.variables.push(constructs::variable::Variable);
        let def = Box::new(CVariable {
            id,
            invariants: invariants.clone(),
            capturing: false,
        });
        let con = c.env.push_construct(def, invariants.clone());
        for inv in invariants {
            let old_scope = c.env.get_construct_scope(inv);
            let new_scope = SVariableInvariants(con);
            c.env.change_scope(old_scope, Box::new(new_scope));
        }
        c.env.check(con);
        TransformerResult(Token::Construct(con))
    }

    fn vomit<'x>(&self, _c: &mut ApplyContext<'_, 'x>, _to: &Token<'x>) -> Option<Token<'x>> {
        None
    }
}
