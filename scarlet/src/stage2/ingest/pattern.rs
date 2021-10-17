use super::rule::Precedence;
use crate::stage1::structure::Token;

#[derive(Clone, Debug)]
pub enum AtomicPat {
    ExactToken(Token<'static>),
    Expression { max_precedence: Precedence },
}

#[derive(Clone, Debug)]
pub enum Pattern {
    Atomic(AtomicPat),
    Composite(Vec<Pattern>),
    Repeat(Box<Pattern>),
}

pub fn rep(base: Pattern) -> Pattern {
    Pattern::Repeat(Box::new(base))
}

impl From<Token<'static>> for Pattern {
    fn from(token: Token<'static>) -> Self {
        Self::Atomic(AtomicPat::ExactToken(token))
    }
}

impl From<Precedence> for Pattern {
    fn from(max_precedence: Precedence) -> Self {
        Self::Atomic(AtomicPat::Expression { max_precedence })
    }
}

impl From<Vec<Pattern>> for Pattern {
    fn from(parts: Vec<Pattern>) -> Self {
        Self::Composite(parts)
    }
}

#[macro_export]
macro_rules! pattern {
    ([$($pat:expr),*]) => {
        vec![$(pattern!($pat)),*].into()
    };
    ($pat:expr) => {
        $pat.into()
    };
}
