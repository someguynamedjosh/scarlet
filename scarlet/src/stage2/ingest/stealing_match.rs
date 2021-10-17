use super::{
    pattern::{AtomicPat, Pattern},
    rule::{Precedence, Rule},
    structs::{MatchComp, PatternMatch, RuleMatch, RuleMatcher},
};

impl<'a, 't> RuleMatcher<'a, 't> {
    pub(super) fn atomic_is_stealing_match<'b>(
        &self,
        pattern: &AtomicPat,
        component: &'b MatchComp<'t>,
        parent_rule_precedence: Precedence,
    ) -> Option<&'b MatchComp<'t>> {
        match component {
            MatchComp::RuleMatch(matchh) => {
                let first_element = &matchh.elements[0];
                match first_element.0 {
                    AtomicPat::ExactToken(..) => None,
                    AtomicPat::Expression { max_precedence } => {
                        if parent_rule_precedence > max_precedence {
                            None
                        } else {
                            self.atomic_is_plain_match(pattern, &first_element.1)
                        }
                    }
                }
            }
            _ => None,
        }
    }

    pub(super) fn composite_is_stealing_match(
        &self,
        elements: &[Pattern],
        remaining_output: &[MatchComp<'t>],
        parent_rule_precedence: Precedence,
    ) -> Option<PatternMatch<'t>> {
        debug_assert!(elements.len() > 0);
        if elements.len() > remaining_output.len() {
            return None;
        }
        let last = elements.len() - 1;
        let before_steal = self.composite_is_plain_match(&elements[..last], remaining_output)?;
        let remaining_output = &remaining_output[before_steal.len()..];
        let steal = self.pattern_is_stealing_match(
            &elements[last],
            remaining_output,
            parent_rule_precedence,
        )?;
        Some([before_steal, steal].concat())
    }

    pub(super) fn repeat_is_stealing_match(
        &self,
        repeated: &Pattern,
        remaining_output: &[MatchComp<'t>],
        parent_rule_precedence: Precedence,
    ) -> Option<PatternMatch<'t>> {
        let mut result = self.repeat_is_plain_match(repeated, remaining_output)?;
        let remaining_output = &remaining_output[result.len()..];
        let mut matchh =
            self.pattern_is_stealing_match(repeated, remaining_output, parent_rule_precedence)?;
        result.append(&mut matchh);
        Some(result)
    }

    pub(super) fn pattern_is_stealing_match(
        &self,
        pattern: &Pattern,
        remaining_output: &[MatchComp<'t>],
        parent_rule_precedence: Precedence,
    ) -> Option<PatternMatch<'t>> {
        match pattern {
            Pattern::Atomic(pat) => {
                if remaining_output.len() == 0 {
                    None
                } else {
                    let matchh = self.atomic_is_stealing_match(
                        pat,
                        &remaining_output[0],
                        parent_rule_precedence,
                    )?;
                    Some(vec![(pat.clone(), matchh.clone())])
                }
            }
            Pattern::Composite(elements) => {
                self.composite_is_stealing_match(elements, remaining_output, parent_rule_precedence)
            }
            Pattern::Repeat(repeated) => {
                self.repeat_is_stealing_match(repeated, remaining_output, parent_rule_precedence)
            }
        }
    }

    /// Returns a RuleMatch if the given rule matches the current output
    /// without stealing from any existing rule matches.
    pub(super) fn rule_is_stealing_match(&self, rule: &Rule) -> Option<RuleMatch<'t>> {
        self.pattern_is_stealing_match(&rule.pattern, &self.output[..], rule.result_precedence)
            .map(|matchh| RuleMatch {
                elements: matchh,
                name: rule.name.clone(),
                precedence: rule.result_precedence,
            })
    }
}
