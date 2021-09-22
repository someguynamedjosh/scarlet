use std::fmt::{self, Debug, Formatter};

use crate::parse::{indented, statements::Statement};

#[derive(Clone, PartialEq)]
pub enum ConstructBody {
    Statements(Vec<Statement>),
    PlainText(String),
}

impl ConstructBody {
    fn is_plain_text(&self) -> bool {
        match self {
            Self::PlainText(..) => true,
            _ => false,
        }
    }
}

impl Debug for ConstructBody {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Statements(statements) => {
                for s in statements {
                    if f.alternate() {
                        let st = format!("{:#?}", s);
                        write!(f, "{}\n", indented(&st))?;
                    } else {
                        write!(f, " ")?;
                        s.fmt(f)?;
                    }
                }
            }
            Self::PlainText(text) => {
                write!(f, "{}", text)?;
            }
        }
        Ok(())
    }
}
