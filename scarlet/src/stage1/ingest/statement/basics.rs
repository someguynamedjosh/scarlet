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
            let sel = Self { name, value };
            Ok((input, sel))
        }
    }
}

fn expand_variant_shorthand(name: Expression, typee: Expression) -> Is {
    let variant_root = Construct::from_expression("variant", typee.clone());
    let value = Expression {
        root: variant_root,
        others: Vec::new(),
    };
    let sel = Is { name, value };
    sel
}

impl Is {
    pub fn variant_shorthand_parser<'i>() -> impl Parser<'i, Self> {
        |input| {
            let (input, name): (_, Expression) = Expression::parser()(input)?;
            let (input, _) = ws()(input)?;
            let (input, _) = alt((
                tag("vn"),
                tag("variant"),
                tag("variant_of"),
                tag("is_variant_of"),
            ))(input)?;
            let (input, _) = ws()(input)?;
            // TODO: Only take type-related postfix constructs.
            let (input, typee): (_, Expression) = Expression::parser()(input)?;
            let st = expand_variant_shorthand(name, typee);
            Ok((input, st))
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
