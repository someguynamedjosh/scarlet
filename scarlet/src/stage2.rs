pub mod ingest;
pub mod structure {
    use std::fmt::{self, Debug, Formatter};

    use crate::{stage1::structure::Token, util::indented};

    #[derive(Clone)]
    pub enum SyntaxNode<'a> {
        Token(Token<'a>),
        Rule {
            name: String,
            elements: Vec<SyntaxNode<'a>>,
        },
    }

    impl<'a> Debug for SyntaxNode<'a> {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            match self {
                Self::Token(token) => write!(f, "{}", token),
                Self::Rule { name, elements } => {
                    write!(f, "({}:", name)?;
                    let sep = match f.alternate() {
                        true => "\n    ",
                        false => " ",
                    };
                    for element in elements {
                        write!(f, "{}", sep)?;
                        let text = match f.alternate() {
                            true => format!("{:#?}", element),
                            false => format!("{:?}", element),
                        };
                        write!(f, "{}", indented(&text))?;
                    }
                    match f.alternate() {
                        true => write!(f, "\n)"),
                        false => write!(f, ")"),
                    }
                }
            }
        }
    }
}

pub use ingest::ingest;
