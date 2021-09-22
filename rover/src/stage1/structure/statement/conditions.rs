use std::fmt::{self, Debug, Formatter};

use crate::stage1::structure::expression::Expression;

#[derive(Clone, PartialEq)]
pub struct Else {
    pub value: Expression,
}

impl Debug for Else {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "else ")?;
        self.value.fmt(f)
    }
}

#[derive(Clone, PartialEq)]
pub struct PickElif {
    pub condition: Expression,
    pub value: Expression,
}

impl Debug for PickElif {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "if ")?;
        self.condition.fmt(f)?;
        write!(f, ", ")?;
        self.value.fmt(f)
    }
}

#[derive(Clone, PartialEq)]
pub struct PickIf {
    pub condition: Expression,
    pub value: Expression,
}

impl Debug for PickIf {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "if ")?;
        self.condition.fmt(f)?;
        write!(f, ", ")?;
        self.value.fmt(f)
    }
}
