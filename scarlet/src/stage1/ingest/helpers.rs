use super::nom_prelude::*;

pub fn tag_then_ws<'i>(tag_value: &'i str) -> impl Parser<'i, ()> {
    move |input| {
        let (input, _) = tag(tag_value)(input)?;
        let (input, _) = ws()(input)?;
        Ok((input, ()))
    }
}

pub fn ws_then_tag<'i>(tag_value: &'i str) -> impl Parser<'i, ()> {
    move |input| {
        let (input, _) = ws()(input)?;
        let (input, _) = tag(tag_value)(input)?;
        Ok((input, ()))
    }
}

pub fn identifier_parser<'i>() -> impl Parser<'i, &'i str> {
    take_while1(|c: char| c.is_alphanumeric() || c == '_')
}
