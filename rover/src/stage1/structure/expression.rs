use std::fmt::{self, Debug, Formatter};

use super::construct::Construct;

#[derive(Clone, PartialEq)]
pub struct Expression {
    pub root: Construct,
    pub others: Vec<Construct>,
}

impl Expression {
    pub fn expect_ident(&self) -> Result<&str, String> {
        if self.others.len() > 0 {
            todo!("nice error")
        }
        self.root.expect_ident()
    }

    pub fn expect_ident_owned(&self) -> Result<String, String> {
        self.expect_ident().map(String::from)
    }
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
