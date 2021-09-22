mod body;
mod constructors;
mod expect;
pub mod labels;

use std::fmt::{self, Debug, Formatter};

pub use body::*;

#[derive(Clone, PartialEq)]
pub struct Construct {
    pub label: String,
    pub body: ConstructBody,
}

impl Debug for Construct {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}{{", self.label)?;
        if f.alternate() && !self.body.is_plain_text() {
            write!(f, "\n")?;
        }
        self.body.fmt(f)?;
        write!(f, "}}")
    }
}
