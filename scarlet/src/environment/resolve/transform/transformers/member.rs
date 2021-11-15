use crate::{
    environment::resolve::transform::{
        apply,
        basics::{ApplyContext, Extras, Transformer, TransformerResult},
        pattern::{
            PatCaptureAny, PatCaptureStream, PatFirstOf, PatPlain, Pattern, PatternMatchSuccess,
        },
    },
    tokens::structure::Token,
};

pub struct Member;

impl Transformer for Member {
    fn pattern(&self) -> Box<dyn Pattern> {
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
        let base = c.env.push_token(base);
        let member_name = success.get_capture("member_name").unwrap_plain();
        let def = todo!(); //Definition::Member(base, StructureMember::Named(member_name.to_owned()));
        let item = c.env.push_def(def);
        TransformerResult(Token::Item(item))
    }
}
