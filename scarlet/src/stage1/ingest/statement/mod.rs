use super::nom_prelude::*;
use crate::stage1::structure::{
    expression::Expression,
    statement::{Else, Is, Parameter, PickElif, PickIf, Replace, Statement},
};

mod basics;
mod conditions;
mod helpers;

impl Statement {
    pub fn parser<'i>() -> impl Parser<'i, Vec<Self>> {
        |input| {
            if let Ok((input, (a, b))) = Parameter::definition_shorthand_parser()(input) {
                let a = Statement::Parameter(a);
                let b = Statement::Is(b);
                Ok((input, vec![a, b]))
            } else {
                let (input, s) = Self::single_parser()(input)?;
                Ok((input, vec![s]))
            }
        }
    }

    fn single_parser<'i>() -> impl Parser<'i, Self> {
        alt((
            map(Parameter::parser(), Statement::Parameter),
            map(Else::parser(), Statement::Else),
            map(Is::parser(), Statement::Is),
            map(Is::variant_shorthand_parser(), Statement::Is),
            map(PickIf::parser(), Statement::PickIf),
            map(PickElif::parser(), Statement::PickElif),
            map(Replace::parser(), Statement::Replace),
            map(Expression::parser(), Statement::Expression),
        ))
    }
}
