use itertools::Itertools;
use maplit::hashmap;

use crate::{
    constructs::{
        self,
        base::ConstructDefinition,
        downcast_construct,
        structt::{self, CEmptyStruct, CPopulatedStruct, SFieldAndRest, SField},
        variable::CVariable,
    },
    tfers,
    tokens::structure::Token,
    transform::{
        apply,
        basics::{ApplyContext, Transformer, TransformerResult},
        pattern::{PatCaptureStream, PatFirstOf, PatPlain, Pattern, PatternMatchSuccess},
        transformers::operators::Is,
    },
};

pub struct SubExpression;
impl Transformer for SubExpression {
    fn input_pattern(&self) -> Box<dyn Pattern> {
        Box::new(PatCaptureStream {
            key: "sub_expression",
            label: "group()",
        })
    }

    fn apply<'t>(
        &self,
        c: &mut ApplyContext<'_, 't>,
        success: PatternMatchSuccess<'_, 't>,
    ) -> TransformerResult<'t> {
        let mut body = success
            .get_capture("sub_expression")
            .unwrap_stream()
            .clone();
        apply::apply_transformers(c, &mut body, &Default::default());
        assert_eq!(body.len(), 1);
        TransformerResult(body.into_iter().next().unwrap())
    }

    fn vomit<'x>(&self, _c: &mut ApplyContext<'_, 'x>, _to: &Token<'x>) -> Option<Token<'x>> {
        None
    }
}

pub struct EmptyStruct;
impl Transformer for EmptyStruct {
    fn input_pattern(&self) -> Box<dyn Pattern> {
        Box::new(PatPlain("EMPTY_STRUCT"))
    }

    fn apply<'t>(
        &self,
        c: &mut ApplyContext<'_, 't>,
        success: PatternMatchSuccess<'_, 't>,
    ) -> TransformerResult<'t> {
        let con = CEmptyStruct;
        let con_id = c.env.push_construct(Box::new(con), vec![]);
        TransformerResult(Token::Construct(con_id))
    }

    fn vomit<'x>(&self, c: &mut ApplyContext<'_, 'x>, to: &Token<'x>) -> Option<Token<'x>> {
        None
    }
}

pub struct PopulatedStruct;
impl Transformer for PopulatedStruct {
    fn input_pattern(&self) -> Box<dyn Pattern> {
        Box::new((
            PatPlain("POPULATED_STRUCT"),
            PatCaptureStream {
                key: "args",
                label: "group[]",
            },
        ))
    }

    fn apply<'t>(
        &self,
        c: &mut ApplyContext<'_, 't>,
        success: PatternMatchSuccess<'_, 't>,
    ) -> TransformerResult<'t> {
        let mut contents = success.get_capture("args").unwrap_stream().clone();
        apply::apply_transformers(c, &mut contents, &Default::default());
        assert_eq!(contents.len(), 3);
        let label = contents[0].unwrap_plain().to_owned();
        let value = c.push_unresolved(contents[1].clone());
        let rest = c.push_unresolved(contents[2].clone());
        let def = Box::new(CPopulatedStruct { label, value, rest });
        let con = c.env.push_construct(def, vec![value, rest]);

        let new_scope = Box::new(SFieldAndRest(con));
        let old_scope = c.env.get_construct_scope(value);
        c.env.change_scope(old_scope, new_scope);

        let new_scope = Box::new(SField(con));
        let old_scope = c.env.get_construct_scope(rest);
        c.env.change_scope(old_scope, new_scope);

        c.env.check(con);
        TransformerResult(Token::Construct(con))
    }

    fn vomit<'x>(&self, c: &mut ApplyContext<'_, 'x>, to: &Token<'x>) -> Option<Token<'x>> {
        if let &Token::Construct(con_id) = to {
            if let Some(structt) =
                downcast_construct::<CPopulatedStruct>(&**c.env.get_construct(con_id))
            {
                let CPopulatedStruct { label, value, rest } = structt;
                let contents = vec![structt.label.clone().into(), value.into(), rest.into()];
                return Some(Token::Stream {
                    label: "group[]",
                    contents,
                });
            }
        }
        None
    }
}

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
