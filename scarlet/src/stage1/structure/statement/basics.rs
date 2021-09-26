use std::fmt::{self, Debug, Formatter};

use crate::stage1::structure::expression::Expression;

#[derive(Clone, PartialEq)]
pub struct Is {
    pub name: Expression,
    pub value: Expression,
}

impl Debug for Is {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.name.fmt(f)?;
        write!(f, " is ")?;
        self.value.fmt(f)
    }
}

#[derive(Clone, PartialEq)]
pub struct Parameter(pub Expression);

impl Debug for Parameter {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "parameter ")?;
        self.0.fmt(f)
    }
}

#[derive(Clone, PartialEq)]
pub struct Replace {
    pub target: Expression,
    pub value: Expression,
}

impl Debug for Replace {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "replace ")?;
        self.target.fmt(f)?;
        write!(f, " with ")?;
        self.value.fmt(f)
    }
}
