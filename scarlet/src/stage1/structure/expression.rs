use std::fmt::{self, Debug, Formatter};

use super::construct::Construct;

#[derive(Clone, PartialEq)]
pub struct Expression {
    pub pres: Vec<Construct>,
    pub root: Construct,
    pub posts: Vec<Construct>,
}

impl Expression {
    pub fn expect_ident(&self) -> Result<&str, String> {
        if !self.posts.is_empty() {
            todo!("nice error")
        }
        self.root.expect_ident()
    }
}

impl Debug for Expression {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.root.fmt(f)?;
        for con in &self.posts {
            if f.alternate() {
                writeln!(f)?;
            } else {
                write!(f, " ")?;
            }
            con.fmt(f)?;
        }
        Ok(())
    }
}
