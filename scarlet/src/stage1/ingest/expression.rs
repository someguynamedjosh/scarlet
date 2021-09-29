use super::nom_prelude::*;
use crate::stage1::structure::{construct::Construct, expression::Expression};

impl Expression {
    pub fn parser<'i>() -> impl Parser<'i, Self> {
        |input| {
            let (input, root) = Construct::parser(true)(input)?;
            let (input, others) = Self::postfixes_parser()(input)?;
            let expr = Self { root, others };
            Ok((input, expr))
        }
    }

    pub fn type_annotation_parser<'i>() -> impl Parser<'i, Self> {
        |input| {
            let (input, root) = Construct::parser(true)(input)?;
            let (input, others) =
                many0(after_ws(Construct::type_annotation_postfix_parser()))(input)?;
            let expr = Self { root, others };
            Ok((input, expr))
        }
    }

    pub fn postfixes_parser<'i>() -> impl Parser<'i, Vec<Construct>> {
        many0(after_ws(Construct::parser(false)))
    }
}
