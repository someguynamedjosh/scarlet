use super::{
    pattern::{AtomicPat, Pattern},
    rule::{Precedence, Rule},
    structs::{MatchComp, PatternMatch, RuleMatch, RuleMatcher},
};

impl<'a, 't> RuleMatcher<'a, 't> {
    pub(super) fn atomic_is_plain_match<'b>(
        &self,
        pattern: &AtomicPat,
        component: &'b MatchComp<'t>,
    ) -> Option<&'b MatchComp<'t>> {
        let matches = match (pattern, component) {
            (AtomicPat::ExactToken(expected), MatchComp::Token(actual)) => expected == actual,
            (AtomicPat::Expression { .. }, MatchComp::Token(..)) => true,
            (AtomicPat::Expression { max_precedence }, MatchComp::RuleMatch(matchh)) => {
                matchh.precedence <= *max_precedence
            }
            _ => false,
        };
        if matches {
            Some(component)
        } else {
            None
        }
    }

    pub(super) fn composite_is_plain_match(
        &self,
        elements: &[Pattern],
        remaining_output: &[MatchComp<'t>],
    ) -> Option<PatternMatch<'t>> {
        if elements.len() > remaining_output.len() {
            return None;
        }
        let mut result = PatternMatch::new();
        let mut remaining_output = remaining_output;
        for element in elements {
            let mut matchh = self.pattern_is_plain_match(element, remaining_output)?;
            remaining_output = &remaining_output[matchh.len()..];
            result.append(&mut matchh);
        }
        Some(result)
    }

    pub(super) fn repeat_is_plain_match(
        &self,
        repeated: &Pattern,
        remaining_output: &[MatchComp<'t>],
    ) -> Option<PatternMatch<'t>> {
        let mut result = PatternMatch::new();
        let mut remaining_output = remaining_output;
        while let Some(mut matchh) = self.pattern_is_plain_match(repeated, remaining_output) {
            remaining_output = &remaining_output[matchh.len()..];
            result.append(&mut matchh);
        }
        Some(result)
    }

    pub(super) fn pattern_is_plain_match(
        &self,
        pattern: &Pattern,
        remaining_output: &[MatchComp<'t>],
    ) -> Option<PatternMatch<'t>> {
        match pattern {
            Pattern::Atomic(pat) => {
                if remaining_output.len() == 0 {
                    None
                } else {
                    let matchh = self.atomic_is_plain_match(pat, &remaining_output[0])?.clone();
                    Some(vec![(pat.clone(), matchh)])
                }
            }
            Pattern::Composite(elements) => {
                self.composite_is_plain_match(elements, remaining_output)
            }
            Pattern::Repeat(repeated) => self.repeat_is_plain_match(repeated, remaining_output),
        }
    }

    /// Returns a RuleMatch if the given rule matches the current output
    /// without stealing from any existing rule matches.
    pub(super) fn rule_is_plain_match(&self, rule: &Rule) -> Option<RuleMatch<'t>> {
        self.pattern_is_plain_match(&rule.pattern, &self.output[..])
            .map(|matchh| RuleMatch {
                elements: matchh,
                name: rule.name.clone(),
                precedence: rule.result_precedence,
            })
    }
}
