use super::{rule::Precedence, structs::MatchComp};
use crate::stage1::structure::Token;

// pub enum AtomicPat {
//     ExactToken(Token<'static>),
//     Expression { max_precedence: Precedence },
// }

pub type AtomicPat = Box<dyn Fn(&MatchComp) -> bool>;

pub enum Pattern {
    Atomic(AtomicPat),
    Composite(Vec<Pattern>),
    Repeat(Box<Pattern>),
}

pub fn rep(base: impl Into<Pattern>) -> Pattern {
    Pattern::Repeat(Box::new(base.into()))
}

impl From<Token<'static>> for Pattern {
    fn from(token: Token<'static>) -> Self {
        Self::Atomic(Box::new(move |test_against| match test_against {
            MatchComp::Token(actual_token) => token == *actual_token,
            MatchComp::RuleMatch(_) => false,
        }))
    }
}

impl From<Precedence> for Pattern {
    fn from(max_precedence: Precedence) -> Self {
        Self::Atomic(Box::new(move |test_against| match test_against {
            MatchComp::Token(_) => true,
            MatchComp::RuleMatch(matchh) => matchh.rule.result_precedence <= max_precedence,
        }))
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
