use crate::stage1::{
    ingest::{
        construct::{explicit, helpers, root_shorthands},
        nom_prelude::*,
    },
    structure::{construct::Construct, expression::Expression},
};

pub fn member_parser<'i>() -> impl Parser<'i, Construct> {
    |input| {
        let (input, _) = tag("::")(input)?;
        let (input, _) = ws()(input)?;
        let (input, root) = alt((explicit::parser(true), root_shorthands::ident_parser()))(input)?;

        let expression = Expression {
            root,
            others: vec![],
        };
        let construct = Construct::from_expression("member", expression);
        Ok((input, construct))
    }
}

fn followed_by_nonempty_whitespace<'i, T>(og: impl Parser<'i, T>) -> impl Parser<'i, T> {
    terminated(og, one_of("\t\r\n "))
}

/// Parses either : or ix, returning true in the latter case to indicate that it
/// is an exact type assertion.
fn type_is_symbol_parser<'i>() -> impl Parser<'i, bool> {
    alt((
        value(true, tag(":")),
        value(true, followed_by_nonempty_whitespace(tag("t"))),
        value(false, followed_by_nonempty_whitespace(tag("bt"))),
    ))
}

pub fn type_is_parser<'i>() -> impl Parser<'i, Construct> {
    |input| {
        let (input, exact) = type_is_symbol_parser()(input)?;
        let (input, _) = ws()(input)?;
        let (input, typee) = Expression::type_annotation_parser()(input)?;

        let label = if exact { "type_is" } else { "base_type_is" };
        Ok((input, Construct::from_expression(label, typee)))
    }
}

pub fn substituting_parser<'i>() -> impl Parser<'i, Construct> {
    |input| {
        let (input, _) = tag("[")(input)?;
        let (input, _) = ws()(input)?;
        let (input, body) = helpers::statement_body_parser()(input)?;
        let (input, _) = helpers::ws_then_tag("]")(input)?;

        Ok((input, Construct::from_body("substituting", body)))
    }
}
