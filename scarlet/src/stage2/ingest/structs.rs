use super::{pattern::AtomicPat, rule::Precedence};
use crate::stage1::structure::Token;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MatchComp {
    Token(usize),
    RuleMatch(usize),
}

pub type PatternMatch = Vec<(AtomicPat, MatchComp)>;

#[derive(Clone, Debug)]
pub struct RuleMatch {
    pub elements: PatternMatch,
    pub name: String,
    pub precedence: Precedence,
}

#[derive(Clone, Debug)]
pub struct RuleMatcher<'a, 't> {
    pub output: Vec<MatchComp>,
    pub tokens: &'a [Token<'t>],
    pub matches: Vec<RuleMatch>,
}

impl<'a, 't> RuleMatcher<'a, 't> {
    pub(super) fn new(tokens: &'a [Token<'t>]) -> Self {
        Self {
            output: Vec::new(),
            tokens,
            matches: Vec::new(),
        }
    }
}
