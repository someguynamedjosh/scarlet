use crate::{
    shared::OrderedSet,
    stage2::{
        reduce::substitute::Substitutions,
        structure::{
            BuiltinOperation, BuiltinPattern, BuiltinValue, Definition, Environment, ItemId,
        },
    },
};

#[derive(Clone, Debug)]
pub enum MatchResult<'x> {
    Match(Substitutions<'x>),
    NoMatch,
    Unknown,
}

use MatchResult::*;

use super::structure::VariableItemIds;

fn non_capturing_match<'x>() -> MatchResult<'x> {
    Match(Substitutions::new())
}

impl<'x> Environment<'x> {
    pub fn matches(
        &mut self,
        original_value: ItemId<'x>,
        match_against: ItemId<'x>,
    ) -> MatchResult<'x> {
        self.matches_impl(
            original_value,
            original_value,
            match_against,
            Default::default(),
        )
    }

    fn matches_impl(
        &mut self,
        original_value: ItemId<'x>,
        value_pattern: ItemId<'x>,
        match_against: ItemId<'x>,
        after: OrderedSet<VariableItemIds<'x>>,
    ) -> MatchResult<'x> {
        let after = after.union(self.get_afters(match_against));
        if let Definition::Variable { matches, .. } = self.definition_of(value_pattern) {
            let matches = *matches;
            return self.matches_impl(original_value, matches, match_against, after);
        }
        match self.definition_of(match_against) {
            Definition::After { base, .. } => {
                // Afters already included using above code.
                let base = *base;
                self.matches_impl(original_value, value_pattern, base, after)
            }
            Definition::BuiltinOperation(op, _) => match op {
                BuiltinOperation::Dif32U | BuiltinOperation::Sum32U => Unknown,
            },
            Definition::BuiltinPattern(BuiltinPattern::God) => non_capturing_match(),
            Definition::BuiltinPattern(BuiltinPattern::And(left, right)) => {
                let (left, right) = (*left, *right);
                let matches_left =
                    self.matches_impl(original_value, value_pattern, left, after.clone());
                let matches_right = self.matches_impl(original_value, value_pattern, right, after);
                match (matches_left, matches_right) {
                    (MatchResult::Match(left), MatchResult::Match(right)) => {
                        MatchResult::Match(left.union(right))
                    }
                    (MatchResult::NoMatch, _) | (_, MatchResult::NoMatch) => MatchResult::NoMatch,
                    _ => MatchResult::Unknown,
                }
            }
            Definition::BuiltinPattern(pat) => {
                let matches = match self.definition_of(value_pattern) {
                    Definition::BuiltinValue(v) => match v {
                        BuiltinValue::Bool(..) => pat == &BuiltinPattern::Bool,
                        BuiltinValue::_32U(..) => pat == &BuiltinPattern::_32U,
                    },
                    Definition::BuiltinPattern(value_pat) => match value_pat {
                        BuiltinPattern::God => return Unknown,
                        BuiltinPattern::Bool => *pat == BuiltinPattern::Bool,
                        BuiltinPattern::_32U => *pat == BuiltinPattern::_32U,
                        BuiltinPattern::And(..) => todo!(),
                    },
                    Definition::BuiltinOperation(value_op, _) => match value_op {
                        BuiltinOperation::Sum32U | BuiltinOperation::Dif32U => {
                            *pat == BuiltinPattern::_32U
                        }
                    },
                    _ => return Unknown,
                };
                if matches {
                    non_capturing_match()
                } else {
                    NoMatch
                }
            }
            Definition::BuiltinValue(pv) => match self.definition_of(value_pattern) {
                Definition::BuiltinValue(vv) => {
                    if pv == vv {
                        // If the pattern of the value being matched is exactly
                        // the pattern we're looking for, it matches.
                        non_capturing_match()
                    } else {
                        // Otherwise, the value matches a specific pattern which
                        // is not a sub-pattern of what we're looking for.
                        NoMatch
                    }
                }
                Definition::Struct(..) => NoMatch,
                _ => Unknown,
            },
            Definition::Match { .. } => Unknown,
            Definition::Member(_, _) => Unknown,
            Definition::Other(other) => {
                let other = *other;
                self.matches_impl(original_value, value_pattern, other, after)
            }
            Definition::Struct(_) => todo!(),
            Definition::Substitute(..) => Unknown,
            Definition::Variable { var, matches } => {
                let (var, matches) = (*var, *matches);
                let mut allow_binding = true;
                for (after, _) in &after {
                    if after.var == var {
                        allow_binding = false;
                        break;
                    }
                }
                match self.matches_impl(original_value, value_pattern, matches, after) {
                    Match(..) => {
                        if allow_binding {
                            let mut subs = Substitutions::new();
                            subs.insert_no_replace(var, original_value);
                            Match(subs)
                        } else {
                            Unknown
                        }
                    }
                    other => other,
                }
            }
        }
    }
}
