use super::{
    pattern::{AtomicPat, Pattern},
    rule::{Precedence, Rule},
    structs::{MatchComp, PatternMatch, RuleMatch, RuleMatcher},
};

impl<'x, 't> RuleMatcher<'x, 't> {
    pub(super) fn atomic_is_stealing_match<'b>(
        &self,
        pattern: &AtomicPat,
        component: &'b MatchComp<'x, 't>,
        parent_rule: &'x Rule,
    ) -> Option<&'b MatchComp<'x, 't>> {
        match component {
            MatchComp::RuleMatch(matchh) => {
                let first_element = &matchh.elements[0];
                let hypothetical_result = MatchComp::RuleMatch(RuleMatch {
                    rule: parent_rule,
                    elements: vec![],
                });
                let we_could_replace_first_element = first_element.0(&hypothetical_result);
                if we_could_replace_first_element {
                    Some(component)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub(super) fn composite_is_stealing_match(
        &self,
        elements: &'x [Pattern],
        remaining_output: &[MatchComp<'x, 't>],
        parent_rule: &'x Rule,
    ) -> Option<PatternMatch<'x, 't>> {
        debug_assert!(elements.len() > 0);
        if elements.len() > remaining_output.len() {
            return None;
        }
        let last = elements.len() - 1;
        let before_steal = self.composite_is_plain_match(&elements[..last], remaining_output)?;
        let remaining_output = &remaining_output[before_steal.len()..];
        let steal =
            self.pattern_is_stealing_match(&elements[last], remaining_output, parent_rule)?;
        Some([before_steal, steal].concat())
    }

    pub(super) fn repeat_is_stealing_match(
        &self,
        repeated: &'x Pattern,
        remaining_output: &[MatchComp<'x, 't>],
        parent_rule: &'x Rule,
    ) -> Option<PatternMatch<'x, 't>> {
        let mut result = self.repeat_is_plain_match(repeated, remaining_output)?;
        let remaining_output = &remaining_output[result.len()..];
        let mut matchh = self.pattern_is_stealing_match(repeated, remaining_output, parent_rule)?;
        result.append(&mut matchh);
        Some(result)
    }

    pub(super) fn pattern_is_stealing_match(
        &self,
        pattern: &'x Pattern,
        remaining_output: &[MatchComp<'x, 't>],
        parent_rule: &'x Rule,
    ) -> Option<PatternMatch<'x, 't>> {
        match pattern {
            Pattern::Atomic(pat) => {
                if remaining_output.len() == 0 {
                    None
                } else {
                    let matchh =
                        self.atomic_is_stealing_match(pat, &remaining_output[0], parent_rule)?;
                    Some(vec![(pat, matchh.clone())])
                }
            }
            Pattern::Composite(elements) => {
                self.composite_is_stealing_match(elements, remaining_output, parent_rule)
            }
            Pattern::Repeat(repeated) => {
                self.repeat_is_stealing_match(repeated, remaining_output, parent_rule)
            }
        }
    }

    /// Returns a RuleMatch if the given rule matches the current output
    /// without stealing from any existing rule matches.
    pub(super) fn rule_is_stealing_match(&self, rule: &'x Rule) -> Option<RuleMatch<'x, 't>> {
        self.pattern_is_stealing_match(&rule.pattern, &self.output[..], rule)
            .map(|elements| RuleMatch { rule, elements })
    }
}
