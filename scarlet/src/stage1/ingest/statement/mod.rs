use super::nom_prelude::*;
use crate::stage1::structure::{
    expression::Expression,
    statement::{Else, PickElif, PickIf, Statement},
};

mod conditions;
mod helpers;

impl Statement {
    pub fn parser<'i>() -> impl Parser<'i, Vec<Self>> {
        |input| {
            let (input, s) = Self::single_parser()(input)?;
            Ok((input, vec![s]))
        }
    }

    fn single_parser<'i>() -> impl Parser<'i, Self> {
        alt((
            map(Else::parser(), Statement::Else),
            map(PickIf::parser(), Statement::PickIf),
            map(PickElif::parser(), Statement::PickElif),
            map(Expression::parser(), Statement::Expression),
        ))
    }
}
