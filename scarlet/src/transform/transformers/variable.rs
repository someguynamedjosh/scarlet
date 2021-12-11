use itertools::Itertools;

use crate::{
    constructs::{
        self,
        structt::{self},
        variable::CVariable,
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
        let invariants = structt::struct_from_unnamed_fields(&mut c.env, invariants);
        let id = c.env.variables.push(constructs::variable::Variable);
        let def = Box::new(CVariable {
            id,
            invariants,
            capturing: false,
        });
        let con = c.env.push_construct(def, vec![invariants]);
        let new_scope = todo!();
        let old_scope = c.env.get_construct_scope(con);
        c.env.change_scope(old_scope, new_scope);
        c.env.check(con);
        TransformerResult(Token::Construct(con))
    }

    fn vomit<'x>(&self, _c: &mut ApplyContext<'_, 'x>, _to: &Token<'x>) -> Option<Token<'x>> {
        None
    }
}
