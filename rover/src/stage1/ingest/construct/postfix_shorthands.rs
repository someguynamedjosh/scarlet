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

/// Parses either : or ix, returning true in the latter case to indicate that it
/// is an exact type assertion.
fn type_is_symbol_parser<'i>() -> impl Parser<'i, bool> {
    alt((value(false, tag(":")), value(true, tag("ix"))))
}

pub fn type_is_parser<'i>() -> impl Parser<'i, Construct> {
    |input| {
        let (input, exact) = type_is_symbol_parser()(input)?;
        let (input, _) = ws()(input)?;
        let (input, typee) = Expression::parser()(input)?;

        let label = if exact { "type_is_exactly" } else { "type_is" };
        Ok((input, Construct::from_expression(label, typee)))
    }
}

pub fn replacing_parser<'i>() -> impl Parser<'i, Construct> {
    |input| {
        let (input, _) = tag("[")(input)?;
        let (input, _) = ws()(input)?;
        let (input, body) = helpers::statement_body_parser()(input)?;
        let (input, _) = helpers::ws_then_tag("]")(input)?;

        Ok((input, Construct::from_body("replacing", body)))
    }
}
