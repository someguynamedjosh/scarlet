use super::nom_prelude::*;
use crate::stage1::structure::{
    expression::Expression,
    statement::{Else, Is, PickElif, PickIf, Replace, Statement},
};

mod basics;
mod conditions;
mod helpers;

impl Statement {
    pub fn parser<'i>() -> impl Parser<'i, Self> {
        alt((
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
