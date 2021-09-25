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

fn expand_variant_shorthand(mut shorthand: Expression) -> Result<Is, String> {
    let name = shorthand.root;
    let name = Expression {
        root: name,
        others: vec![],
    };
    if shorthand.others.is_empty() {
        return Err(format!(""));
    }
    let typee_construct = shorthand.others.remove(0);
    let typee = typee_construct.expect_single_expression("type_is")?;
    let variant_root = Construct::from_expression("variant", typee.clone());
    let value = Expression {
        root: variant_root,
        others: shorthand.others,
    };
    let sel = Is { name, value };
    Ok(sel)
}

impl Is {
    pub fn variant_shorthand_parser<'i>() -> impl Parser<'i, Self> {
        |input| {
            let (input, _) = helpers::tag_then_ws("variant")(input)?;
            let (input, shorthand): (_, Expression) = Expression::parser()(input)?;
            match expand_variant_shorthand(shorthand) {
                Ok(res) => Ok((input, res)),
                Err(..) => fail(input),
            }
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
