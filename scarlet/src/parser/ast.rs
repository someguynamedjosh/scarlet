use std::fmt::{self, Debug, Formatter};

use super::token::Token;
use crate::shared::indented;

#[derive(Clone)]
pub enum AstNode<'a> {
    Rule {
        // rule: &'a Rule,
        components: Vec<AstNode<'a>>,
    },
    Terminal(&'a Token<'a>),
}

impl<'a> Debug for AstNode<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AstNode::Rule { components } => {
                writeln!(f, "{:?} [", ())?;
                for component in components {
                    writeln!(f, "    {},", indented(&format!("{:?}", component)))?;
                }
                write!(f, "]")
            }
            AstNode::Terminal(token) => token.fmt(f),
        }
    }
}
