use std::fmt::{self, Debug, Formatter};

use super::expression::Expression;

mod basics;
mod conditions;

pub use basics::*;
pub use conditions::*;

#[derive(Clone, PartialEq)]
pub enum Statement {
    Else(Else),
    Expression(Expression),
    Is(Is),
    PickIf(PickIf),
    PickElif(PickElif),
    Replace(Replace),
}

impl Debug for Statement {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Else(s) => s.fmt(f),
            Self::Expression(s) => s.fmt(f),
            Self::Is(s) => s.fmt(f),
            Self::PickIf(s) => s.fmt(f),
            Self::PickElif(s) => s.fmt(f),
            Self::Replace(s) => s.fmt(f),
        }
    }
}
