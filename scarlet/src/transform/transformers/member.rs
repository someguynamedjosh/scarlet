use crate::{
    constructs::{downcast_construct, member::CMember},
    tokens::structure::Token,
    transform::{
        basics::{ApplyContext, Transformer, TransformerResult},
        pattern::{PatCaptureAny, PatPlain, Pattern, PatternMatchSuccess},
    },
};

pub struct Member;

impl Transformer for Member {
    fn input_pattern(&self) -> Box<dyn Pattern> {
        Box::new((
            PatCaptureAny { key: "base" },
            PatPlain("."),
            PatCaptureAny { key: "member_name" },
        ))
    }

    fn apply<'t>(
        &self,
        c: &mut ApplyContext<'_, 't>,
        success: PatternMatchSuccess<'_, 't>,
    ) -> TransformerResult<'t> {
        let base = success.get_capture("base").clone();
        let base = c.push_unresolved(base);
        let member_name = success.get_capture("member_name").unwrap_plain();
        let def = CMember(base, member_name.to_owned());
        let con = c.env.push_construct(Box::new(def));
        TransformerResult(Token::Construct(con))
    }

    fn vomit<'x>(&self, c: &mut ApplyContext<'_, 'x>, to: &Token<'x>) -> Option<Vec<Token<'x>>> {
        if let &Token::Construct(con_id) = to {
            if let Some(mem) = downcast_construct::<CMember>(&**c.env.get_construct(con_id)) {
                return Some(vec![mem.0.into(), ".".into(), mem.1.clone().into()]);
            }
        }
        None
    }
}
