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
