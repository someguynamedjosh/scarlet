use std::fmt::{self, Debug, Formatter};

use super::construct::Construct;

#[derive(Clone, PartialEq)]
pub struct Expression {
    pub root: Construct,
    pub others: Vec<Construct>,
}

impl Debug for Expression {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.root.fmt(f)?;
        for con in &self.others {
            if f.alternate() {
                write!(f, "\n")?;
            } else {
                write!(f, " ")?;
            }
            con.fmt(f)?;
        }
        Ok(())
    }
}
