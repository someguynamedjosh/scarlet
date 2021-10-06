use std::fmt::{self, Debug, Formatter};

use crate::{stage1::structure::expression::Expression, util};

#[derive(Clone, PartialEq)]
pub enum ConstructBody {
    Expressions(Vec<Expression>),
    PlainText(String),
}

impl ConstructBody {
    pub fn is_plain_text(&self) -> bool {
        match self {
            Self::PlainText(..) => true,
            _ => false,
        }
    }

    pub fn expect_text(&self) -> Result<&str, String> {
        match self {
            Self::PlainText(txt) => Ok(txt),
            _ => todo!("nice error"),
        }
    }
}

fn fmt_expressions(f: &mut Formatter, expressions: &[Expression]) -> fmt::Result {
    let mut first = true;
    for s in expressions {
        if f.alternate() {
            if !first {
                write!(f, "\n")?;
            }
            let st = format!("{:#?}", s);
            write!(f, "\n    {}", util::indented(&st))?;
        } else {
            write!(f, " ")?;
            s.fmt(f)?;
        }
        first = false;
    }
    Ok(())
}

impl Debug for ConstructBody {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Expressions(expressions) => fmt_expressions(f, expressions),
            Self::PlainText(text) => write!(f, "{}", text),
        }
    }
}
