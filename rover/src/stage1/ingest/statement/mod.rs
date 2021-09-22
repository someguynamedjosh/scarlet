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
            map(Else::parser(), |s| Statement::Else(s)),
            map(Is::parser(), |s| Statement::Is(s)),
            map(Is::variant_shorthand_parser(), |s| Statement::Is(s)),
            map(PickIf::parser(), |s| Statement::PickIf(s)),
            map(PickElif::parser(), |s| Statement::PickElif(s)),
            map(Replace::parser(), |s| Statement::Replace(s)),
            map(Expression::parser(), |s| Statement::Expression(s)),
        ))
    }
}
