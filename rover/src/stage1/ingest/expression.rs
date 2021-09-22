use super::nom_prelude::*;
use crate::stage1::structure::{construct::Construct, expression::Expression};

impl Expression {
    pub fn parser<'i>() -> impl Parser<'i, Self> {
        |input| {
            let (input, root) = Construct::parser(true)(input)?;
            let (input, others) = many0(after_ws(Construct::parser(false)))(input)?;
            let expr = Self { root, others };
            Ok((input, expr))
        }
    }
}
