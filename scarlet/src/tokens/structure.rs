use std::{
    borrow::Cow,
    fmt::{self, Debug, Formatter},
};

use crate::{constructs::base::ConstructId, shared};

pub type TokenStream<'x> = Vec<Token<'x>>;

#[derive(Clone, PartialEq, Eq)]
pub enum Token<'x> {
    Construct(ConstructId),
    Plain(Cow<'x, str>),
    Stream {
        label: &'x str,
        contents: TokenStream<'x>,
    },
}

impl<'x> From<&'x str> for Token<'x> {
    fn from(contents: &'x str) -> Self {
        Self::Plain(Cow::Borrowed(contents))
    }
}

impl<'x> From<String> for Token<'x> {
    fn from(contents: String) -> Self {
        Self::Plain(Cow::Owned(contents))
    }
}

impl<'a, 'x> From<&'a ConstructId> for Token<'x> {
    fn from(input: &'a ConstructId) -> Self {
        Self::Construct(*input)
    }
}

impl<'x> From<ConstructId> for Token<'x> {
    fn from(contents: ConstructId) -> Self {
        Self::Construct(contents)
    }
}

impl<'x> Token<'x> {
    pub fn unwrap_construct(&self) -> ConstructId {
        if let Self::Construct(con) = self {
            *con
        } else {
            panic!("Expected a construct token, got {:?} instead", self)
        }
    }

    pub fn unwrap_plain(&self) -> &str {
        if let Self::Plain(plain) = self {
            plain.as_ref()
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
