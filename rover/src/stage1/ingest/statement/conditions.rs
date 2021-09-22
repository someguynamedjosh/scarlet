use super::helpers;
use crate::stage1::{
    ingest::nom_prelude::*,
    structure::{
        expression::Expression,
        statement::{Else, PickElif, PickIf},
    },
};

impl Else {
    pub fn parser<'i>() -> impl Parser<'i, Self> {
        |input| {
            let (input, _) = helpers::tag_then_ws("else")(input)?;
            let (input, value) = Expression::parser()(input)?;
            Ok((input, Self { value }))
        }
    }
}

/// Parses input formatted like "tag condition, value"
fn tag_condition_comma_value_parser<'i>(
    tag: &'static str,
) -> impl Parser<'i, (Expression, Expression)> {
    move |input| {
        let (input, _) = helpers::tag_then_ws(tag)(input)?;
        let (input, condition) = Expression::parser()(input)?;
        let (input, _) = ws()(input)?;
        let (input, _) = helpers::tag_then_ws(",")(input)?;
        let (input, value) = Expression::parser()(input)?;
        Ok((input, (condition, value)))
    }
}

impl PickElif {
    pub fn parser<'i>() -> impl Parser<'i, Self> {
        let condition_value = tag_condition_comma_value_parser("elif");
        map(condition_value, |(condition, value)| Self {
            condition,
            value,
        })
    }
}

impl PickIf {
    pub fn parser<'i>() -> impl Parser<'i, Self> {
        let condition_value = tag_condition_comma_value_parser("if");
        map(condition_value, |(condition, value)| Self {
            condition,
            value,
        })
    }
}
