use std::fmt::{self, Debug, Formatter};

use crate::util::indented;

pub type Token<'i> = &'i str;

#[derive(Clone, PartialEq, Eq)]
pub enum TokenTree<'i> {
    Token(Token<'i>),
    Group {
        start: Token<'i>,
        end: Token<'i>,
        body: Vec<TokenTree<'i>>,
    },
    PrimitiveRule {
        name: Token<'i>,
        body: Vec<TokenTree<'i>>,
    },
}

impl<'i> Debug for TokenTree<'i> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TokenTree::Token(text) => write!(f, "{:?}", text),
            TokenTree::Group { start, end, body } => {
                write!(f, "group{}", start)?;
                for tt in body {
                    match f.alternate() {
                        true => write!(f, "\n    {}", indented(&format!("{:#?}", tt)))?,
                        false => write!(f, " {:?}", tt)?,
                    }
                }
                match f.alternate() {
                    true => write!(f, "\n{}", end),
                    false => write!(f, " {}", end),
                }
            }
            TokenTree::PrimitiveRule { name, body } => {
                match f.alternate() {
                    true => write!(f, "primitive{{\n    {:?}", name)?,
                    false => write!(f, "primitive{{{:?}", name)?,
                }
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
