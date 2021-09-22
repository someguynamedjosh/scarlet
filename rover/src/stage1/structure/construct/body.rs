use std::fmt::{self, Debug, Formatter};

use crate::{stage1::structure::statement::Statement, util};

#[derive(Clone, PartialEq)]
pub enum ConstructBody {
    Statements(Vec<Statement>),
    PlainText(String),
}

impl ConstructBody {
    pub fn is_plain_text(&self) -> bool {
        match self {
            Self::PlainText(..) => true,
            _ => false,
        }
    }
}

fn fmt_statements(f: &mut Formatter, statements: &[Statement]) -> fmt::Result {
    for s in statements {
        if f.alternate() {
            let st = format!("{:#?}", s);
            write!(f, "{}\n", util::indented(&st))?;
        } else {
            write!(f, " ")?;
            s.fmt(f)?;
        }
    }
    Ok(())
}

impl Debug for ConstructBody {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Statements(statements) => fmt_statements(f, statements),
            Self::PlainText(text) => write!(f, "{}", text),
        }
    }
}
