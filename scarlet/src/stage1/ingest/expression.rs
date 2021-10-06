use super::nom_prelude::*;
use crate::stage1::structure::{
    construct::{Construct, Position},
    expression::Expression,
};

impl Expression {
    pub fn parser<'i>() -> impl Parser<'i, Self> {
        |input| {
            let (input, pres) = Self::prefixes_parser()(input)?;
            let (input, _) = ws()(input)?;
            let (input, root) = Construct::parser(Position::Root)(input)?;
            let (input, posts) = Self::postfixes_parser()(input)?;
            let expr = Self { pres, root, posts };
            Ok((input, expr))
        }
    }

    pub fn type_annotation_parser<'i>() -> impl Parser<'i, Self> {
        |input| {
            let pres = Vec::new();
            let (input, root) = Construct::parser(Position::Root)(input)?;
            let (input, posts) =
                many0(after_ws(Construct::type_annotation_postfix_parser()))(input)?;
            let expr = Self { pres, root, posts };
            Ok((input, expr))
        }
    }

    pub fn prefixes_parser<'i>() -> impl Parser<'i, Vec<Construct>> {
        many0(after_ws(Construct::parser(Position::Prefix)))
    }

    pub fn postfixes_parser<'i>() -> impl Parser<'i, Vec<Construct>> {
        many0(after_ws(Construct::parser(Position::Postfix)))
    }
}
