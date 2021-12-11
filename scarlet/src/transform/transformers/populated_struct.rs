use crate::{
    constructs::{
        downcast_construct,
        structt::{CPopulatedStruct, SField, SFieldAndRest},
    },
    tokens::structure::Token,
    transform::{
        apply,
        basics::{ApplyContext, Transformer, TransformerResult},
        pattern::{PatCaptureStream, PatPlain, Pattern, PatternMatchSuccess},
    },
};

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
                let CPopulatedStruct {
                    label: _,
                    value,
                    rest,
                } = structt;
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
