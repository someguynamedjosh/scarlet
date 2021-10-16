use crate::stage1::structure::Token;

pub type Precedence = u8;

#[derive(Clone, Debug)]
pub enum AtomicPattern {
    ExactToken(Token<'static>),
    Expression { max_precedence: Precedence },
}

#[derive(Clone, Debug)]
pub enum Pattern {
    Atomic(AtomicPattern),
    Composite(Vec<Pattern>),
    Repeat(Box<Pattern>),
}

fn rep(base: Pattern) -> Pattern {
    Pattern::Repeat(Box::new(base))
}

impl From<Token<'static>> for Pattern {
    fn from(token: Token<'static>) -> Self {
        Self::Atomic(AtomicPattern::ExactToken(token))
    }
}

impl From<Precedence> for Pattern {
    fn from(max_precedence: Precedence) -> Self {
        Self::Atomic(AtomicPattern::Expression { max_precedence })
    }
}

impl From<Vec<Pattern>> for Pattern {
    fn from(parts: Vec<Pattern>) -> Self {
        Self::Composite(parts)
    }
}

macro_rules! pattern {
    ([$($pat:expr),*]) => {
        vec![$(pattern!($pat)),*].into()
    };
    ($pat:expr) => {
        $pat.into()
    };
}

#[derive(Clone, Debug)]
pub struct Rule {
    pub name: String,
    pub pattern: Pattern,
    pub result_precedence: Precedence,
}

pub fn build_rules() -> Vec<Rule> {
    vec![
        Rule {
            name: format!("+"),
            pattern: pattern!([80, "+", 79]),
            result_precedence: 80,
        },
        Rule {
            name: format!("*"),
            pattern: pattern!([70, "*", 69]),
            result_precedence: 70,
        },
        Rule {
            name: format!("paren"),
            pattern: pattern!(["(", rep(pattern!(255)), ")"]),
            result_precedence: 1,
        },
    ]
}
