use super::{pattern::AtomicPat, rule::Precedence};
use crate::stage1::structure::Token;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MatchComp<'t> {
    Token(Token<'t>),
    RuleMatch(RuleMatch<'t>),
}

pub type PatternMatch<'t> = Vec<(AtomicPat, MatchComp<'t>)>;

#[derive(Clone, Debug)]
pub struct RuleMatch<'t> {
    pub elements: PatternMatch<'t>,
    pub name: String,
    pub precedence: Precedence,
}

impl<'t> PartialEq for RuleMatch<'t> {
    fn eq(&self, other: &Self) -> bool {
        if self.elements.len() != other.elements.len() {
            return false;
        }
        for index in 0..self.elements.len() {
            if self.elements[index].1 == other.elements[index].1 {
                return false;
            }
        }
        self.name == other.name && self.precedence == other.precedence
    }
}

impl<'t> Eq for RuleMatch<'t> {}

#[derive(Clone, Debug)]
pub struct RuleMatcher<'a, 't> {
    pub output: Vec<MatchComp<'t>>,
    pub tokens: &'a [Token<'t>],
}

impl<'a, 't> RuleMatcher<'a, 't> {
    pub(super) fn new(tokens: &'a [Token<'t>]) -> Self {
        Self {
            output: Vec::new(),
            tokens,
        }
    }
}
