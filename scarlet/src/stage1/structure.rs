use std::fmt::{self, Debug, Formatter};

use crate::util::indented;

pub type Token<'i> = &'i str;

#[derive(Clone, PartialEq, Eq)]
pub enum TokenTree<'i> {
    Token(Token<'i>),
    PrimitiveRule {
        name: Token<'i>,
        body: Vec<TokenTree<'i>>,
    },
}

impl<'i> TokenTree<'i> {
    pub fn as_token(&self) -> Option<Token<'i>> {
        match self {
            TokenTree::Token(token) => Some(token),
            _ => None,
        }
    }

    pub fn unwrap_primitive(&self, expected_name: Token<'i>) -> &[TokenTree<'i>] {
        match self {
            Self::PrimitiveRule { name, body } => {
                if *name == expected_name {
                    &body[..]
                } else {
                    panic!(
                        "The actual name {} does not match the expected name {}",
                        name, expected_name
                    )
                }
            }
            _ => panic!("Expected a primitive rule, found a token instead."),
        }
    }
}

impl<'i> Debug for TokenTree<'i> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TokenTree::Token(text) => write!(f, "{:?}", text),
            TokenTree::PrimitiveRule { name, body } => {
                write!(f, "{}{{", name)?;
                for tt in body {
                    match f.alternate() {
                        true => write!(f, "\n    {}", indented(&format!("{:#?}", tt)))?,
                        false => write!(f, " {:?}", tt)?,
                    }
                }
                match f.alternate() {
                    true => write!(f, "\n}}"),
                    false => write!(f, "}}"),
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Module<'a> {
    pub self_content: Vec<TokenTree<'a>>,
    pub children: Vec<(String, Module<'a>)>,
}
