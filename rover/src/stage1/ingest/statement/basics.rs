use super::helpers;
use crate::stage1::{
    ingest::nom_prelude::*,
    structure::{
        construct::Construct,
        expression::Expression,
        statement::{Is, Parameter, Replace},
    },
};

impl Is {
    pub fn parser<'i>() -> impl Parser<'i, Self> {
        |input| {
            let (input, name) = Expression::parser()(input)?;
            let (input, _) = ws()(input)?;
            let (input, _) = helpers::tag_then_ws("is")(input)?;
            let (input, value) = Expression::parser()(input)?;
            let sel = Self {
                name,
                value,
            };
            Ok((input, sel))
        }
    }
}

fn variant_shorthand_name(from_shorthand: &Expression) -> Expression {
    Expression {
        root: from_shorthand.root.clone(),
        others: vec![],
    }
}

/// Returns the name and value of a variant to use as a full `is` statement.
fn expand_variant_shorthand(shorthand: Expression) -> (Expression, Expression) {
    let name = variant_shorthand_name(&shorthand);
    let value = Expression {
        root: Construct::from_expression("variant", shorthand),
        others: vec![],
    };
    (name, value)
}

impl Is {
    pub fn variant_shorthand_parser<'i>() -> impl Parser<'i, Self> {
        |input| {
            let (input, _) = helpers::tag_then_ws("variant")(input)?;
            let (input, variant_def) = Expression::parser()(input)?;
            let (name, value) = expand_variant_shorthand(variant_def);
            let sel = Self {
                name,
                value,
            };
            Ok((input, sel))
        }
    }
}

impl Parameter {
    pub fn parser<'i>() -> impl Parser<'i, Self> {
        |input| {
            let (input, _) = alt((tag("parameter"), tag("param"), tag("p")))(input)?;
            let (input, _) = ws()(input)?;
            let (input, name) = Expression::parser()(input)?;
            Ok((input, Self(name)))
        }
    }

    pub fn definition_shorthand_parser<'i>() -> impl Parser<'i, (Self, Is)> {
        |input| {
            let (input, sel) = Self::parser()(input)?;
            let (input, _) = ws()(input)?;
            let (input, _) = helpers::tag_then_ws("is")(input)?;
            let (input, value) = Expression::parser()(input)?;
            let is = Is {
                name: sel.0.clone(),
                value,
            };
            Ok((input, (sel, is)))
        }
    }
}

impl Replace {
    pub fn parser<'i>() -> impl Parser<'i, Self> {
        |input| {
            let (input, target) = Expression::parser()(input)?;
            let (input, _) = ws()(input)?;
            let (input, _) = helpers::tag_then_ws("with")(input)?;
            let (input, value) = Expression::parser()(input)?;
            Ok((input, Self { target, value }))
        }
    }
}
