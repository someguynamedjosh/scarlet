use super::{
    pattern::{AtomicPat, Pattern},
    rule::Rule,
    structs::{MatchComp, PatternMatch, RuleMatch, RuleMatcher},
};

impl<'x, 't> RuleMatcher<'x, 't> {
    pub(super) fn atomic_is_plain_match<'b>(
        &self,
        pattern: &AtomicPat,
        component: &'b MatchComp<'x, 't>,
    ) -> Option<&'b MatchComp<'x, 't>> {
        if pattern(component) {
            Some(component)
        } else {
            None
        }
    }

    pub(super) fn composite_is_plain_match(
        &self,
        elements: &'x [Pattern],
        remaining_output: &[MatchComp<'x, 't>],
    ) -> Option<PatternMatch<'x, 't>> {
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
        repeated: &'x Pattern,
        remaining_output: &[MatchComp<'x, 't>],
    ) -> Option<PatternMatch<'x, 't>> {
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
        pattern: &'x Pattern,
        remaining_output: &[MatchComp<'x, 't>],
    ) -> Option<PatternMatch<'x, 't>> {
        match pattern {
            Pattern::Atomic(pat) => {
                if remaining_output.len() == 0 {
                    None
                } else {
                    let matchh = self.atomic_is_plain_match(pat, &remaining_output[0])?;
                    Some(vec![(pat, matchh.clone())])
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
    pub(super) fn rule_is_plain_match(&self, rule: &'x Rule) -> Option<RuleMatch<'x, 't>> {
        self.pattern_is_plain_match(&rule.pattern, &self.output[..])
            .map(|matchh| RuleMatch {
                rule,
                elements: matchh,
            })
    }
}
