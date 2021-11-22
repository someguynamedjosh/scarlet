use crate::{
    constructs::member::{CMember, Member as ConstructsMember},
    environment::resolve::transform::{
        basics::{ApplyContext, Transformer, TransformerResult},
        pattern::{PatCaptureAny, PatPlain, Pattern, PatternMatchSuccess},
    },
    tokens::structure::Token,
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
        let def = CMember(base, ConstructsMember::Named(member_name.to_owned()));
        let con = c.env.push_construct(Box::new(def));
        TransformerResult(Token::Construct(con))
    }
}
