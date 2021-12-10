use itertools::Itertools;
use maplit::hashmap;

use crate::{
    constructs::{
        self, base::ConstructDefinition, downcast_construct, structt::CPopulatedStruct,
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
        let label = contents.remove(0);
        let label = label.unwrap_plain().to_owned();
        let con = c.push_placeholder();
        let mut c = c.with_parent_scope(Some(con));
        apply::apply_transformers(&mut c, &mut contents, &Default::default());
        assert_eq!(contents.len(), 2);
        let def = Box::new(CPopulatedStruct {
            label,
            value: c.push_unresolved(contents[0].clone()),
            rest: c.push_unresolved(contents[1].clone()),
        });
        c.env.constructs[con].definition = ConstructDefinition::Resolved(def);
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
                label: "group{}",
            },
        ))
    }

    fn apply<'t>(
        &self,
        c: &mut ApplyContext<'_, 't>,
        success: PatternMatchSuccess<'_, 't>,
    ) -> TransformerResult<'t> {
        let mut invariants = success.get_capture("invariants").unwrap_stream().clone();
        let con = c.push_placeholder();
        let mut c = c.with_parent_scope(Some(con));
        apply::apply_transformers(&mut c, &mut invariants, &Default::default());
        let invariants = invariants
            .into_iter()
            .map(|x| c.push_unresolved(x))
            .collect_vec();
        let id = c.env.variables.push(constructs::variable::Variable);
        let def = Box::new(CVariable {
            id,
            invariants,
            capturing: false,
        });
        c.env.constructs[con].definition = ConstructDefinition::Resolved(def);
        c.env.check(con);
        TransformerResult(Token::Construct(con))
    }

    fn vomit<'x>(&self, _c: &mut ApplyContext<'_, 'x>, _to: &Token<'x>) -> Option<Token<'x>> {
        None
    }
}
