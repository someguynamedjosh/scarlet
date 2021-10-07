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

    pub fn extract(&mut self, label: &str) -> Option<Construct> {
        for index in 0..self.pres.len() {
            if self.pres[index].label == label {
                return Some(self.pres.remove(index));
            }
        }
        None
    }

    pub fn extract_single_expression(&mut self, label: &str) -> Option<Result<Expression, String>> {
        match self.extract(label) {
            Some(con) => Some(con.expect_single_expression(label).map(Clone::clone)),
            None => None,
        }
    }

    pub fn extract_target(&mut self) -> Option<Result<Expression, String>> {
        self.extract_single_expression("target")
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
