use std::fmt::{self, Debug, Formatter};

use crate::{environment::ConstructId, shared};

pub type TokenStream<'x> = Vec<Token<'x>>;

#[derive(Clone, PartialEq, Eq)]
pub enum Token<'x> {
    Construct(ConstructId<'x>),
    Plain(&'x str),
    Stream {
        label: &'x str,
        contents: TokenStream<'x>,
    },
}

impl<'x> Token<'x> {
    pub fn unwrap_plain(&self) -> &'x str {
        if let Self::Plain(plain) = self {
            *plain
        } else {
            panic!("Expected a plain token, got {:?} instead", self)
        }
    }

    pub fn unwrap_stream(&self) -> &TokenStream<'x> {
        if let Self::Stream { contents, .. } = self {
            contents
        } else {
            panic!("Expected a stream token, got {:?} instead", self)
        }
    }
}

impl<'x> Debug for Token<'x> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Token::Construct(id) => write!(f, "{:?}", id),
            Token::Plain(plain) => write!(f, "{}", plain),
            Token::Stream { label, contents } => {
                writeln!(f, "stream {} {{", label)?;
                for line in contents {
                    writeln!(f, "    {}", shared::indented(&format!("{:?}", line)))?;
                }
                write!(f, "}}")
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Module<'x> {
    pub self_content: Token<'x>,
    pub children: Vec<(String, Module<'x>)>,
}
