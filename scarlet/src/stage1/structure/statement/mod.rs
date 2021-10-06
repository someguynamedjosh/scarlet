use std::fmt::{self, Debug, Formatter};

use super::expression::Expression;

mod conditions;

pub use conditions::*;

#[derive(Clone, PartialEq)]
pub enum Statement {
    Else(Else),
    Expression(Expression),
    PickIf(PickIf),
    PickElif(PickElif),
}

impl Statement {
    pub fn expect_expression(&self) -> Result<&Expression, String> {
        if let Self::Expression(expr) = self {
            Ok(expr)
        } else {
            todo!("nice error, expected expression")
        }
    }
}

impl Debug for Statement {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Else(s) => s.fmt(f),
            Self::Expression(s) => s.fmt(f),
            Self::PickIf(s) => s.fmt(f),
            Self::PickElif(s) => s.fmt(f),
        }
    }
}
