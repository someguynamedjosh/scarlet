use crate::stage1::{
    ingest::{construct::helpers, nom_prelude::*},
    structure::construct::Construct,
};

pub fn ident_parser<'i>() -> impl Parser<'i, Construct> {
    |input| {
        let (input, name) = helpers::identifier_parser()(input)?;
        let (input, _) = not(helpers::ws_then_tag("{"))(input)?;

        Ok((input, Construct::from_text("identifier", name)))
    }
}

fn remove_underscores(text: &str) -> String {
    text.replace("_", "")
}

/// Returns true if the specified string is an integer literal.
fn is_int(text: &str) -> bool {
    text.parse::<i32>().is_ok()
}

/// Returns Ok() if the specified text is an int literal. The result is the text
/// of the literal with underscores removed.
fn int_literal_parser<'i>() -> impl Parser<'i, String> {
    let text = helpers::identifier_parser();
    let without_underscores = map(text, remove_underscores);

    verify(without_underscores, is_int)
}

pub fn integer_shorthand_parser<'i>() -> impl Parser<'i, Construct> {
    map(int_literal_parser(), |text| {
        Construct::from_text("i32", &text[..])
    })
}
