use super::{
    pattern::AtomicPat,
    rule::{Precedence, Rule},
};
use crate::stage1::structure::Token;

#[derive(Clone, PartialEq, Eq)]
pub enum MatchComp<'r, 't> {
    Token(Token<'t>),
    RuleMatch(RuleMatch<'r, 't>),
}

pub type PatternMatch<'r, 't> = Vec<(&'r AtomicPat, MatchComp<'r, 't>)>;

#[derive(Clone)]
pub struct RuleMatch<'r, 't> {
    pub rule: &'r Rule,
    pub elements: PatternMatch<'r, 't>,
}

impl<'r, 't> PartialEq for RuleMatch<'r, 't> {
    fn eq(&self, other: &Self) -> bool {
        if self.elements.len() != other.elements.len() {
            return false;
        }
        for index in 0..self.elements.len() {
            if self.elements[index].1 == other.elements[index].1 {
                return false;
            }
        }
        self.rule.name == other.rule.name
            && self.rule.result_precedence == other.rule.result_precedence
    }
}

impl<'r, 't> Eq for RuleMatch<'r, 't> {}

pub struct RuleMatcher<'x, 't> {
    pub output: Vec<MatchComp<'x, 't>>,
    pub rules: &'x [Rule],
    pub tokens: &'x [Token<'t>],
}

impl<'x, 't> RuleMatcher<'x, 't> {
    pub(super) fn new(rules: &'x [Rule], tokens: &'x [Token<'t>]) -> Self {
        Self {
            output: Vec::new(),
            rules,
            tokens,
        }
    }
}
