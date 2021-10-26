use crate::{
    shared::OrderedSet,
    stage2::{
        reduce::substitute::Substitutions,
        structure::{BuiltinOperation, BuiltinValue, Definition, Environment, ItemId, VariableId},
    },
};

pub enum MatchResult<'x> {
    Match(Substitutions<'x>),
    NoMatch,
    Unknown,
}

use MatchResult::*;

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
        after: OrderedSet<VariableId<'x>>,
    ) -> MatchResult<'x> {
        let after = after.union(self.get_afters(match_against));
        if let Definition::Variable(var) = self.definition_of(value_pattern) {
            let var_pattern = self.vars[*var].pattern;
            return self.matches_impl(original_value, var_pattern, match_against, after);
        }
        match self.definition_of(match_against) {
            Definition::After { base, .. } => {
                // Afters already included using above code.
                let base = *base;
                self.matches_impl(original_value, value_pattern, base, after)
            }
            Definition::BuiltinOperation(op, _) => match op {
                BuiltinOperation::Dif32U | BuiltinOperation::Sum32U => Unknown,
                BuiltinOperation::_32UPattern | &BuiltinOperation::BoolPattern => {
                    let matches = match self.definition_of(value_pattern) {
                        Definition::BuiltinValue(v) => match v {
                            BuiltinValue::Bool(..) => op == &BuiltinOperation::BoolPattern,
                            BuiltinValue::_32U(..) => op == &BuiltinOperation::_32UPattern,
                            BuiltinValue::GodPattern => false,
                        },
                        Definition::BuiltinOperation(value_op, _) => match value_op {
                            BuiltinOperation::BoolPattern => *op == BuiltinOperation::BoolPattern,
                            BuiltinOperation::_32UPattern
                            | BuiltinOperation::Sum32U
                            | BuiltinOperation::Dif32U => *op == BuiltinOperation::_32UPattern,
                        },
                        _ => return Unknown,
                    };
                    if matches {
                        non_capturing_match()
                    } else {
                        NoMatch
                    }
                }
            },
            Definition::BuiltinValue(BuiltinValue::GodPattern) => non_capturing_match(),
            Definition::BuiltinValue(pv) => match self.definition_of(value_pattern) {
                Definition::BuiltinValue(vv) => {
                    if pv == vv {
                        // If the pattern of the value being matched is exactly
                        // the pattern we're looking for, it matches.
                        non_capturing_match()
                    } else if *vv == BuiltinValue::GodPattern {
                        // Otherwise, if it is the only super pattern possible
                        // in BuiltinValue, then we don't know if the actual
                        // value (matching PATTERN) matches the specific pattern
                        // we're looking for.
                        Unknown
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
            Definition::Variable(var) => {
                let var = *var;
                let allow_binding = !after.contains_key(&var);
                let var_pattern = self.vars[var].pattern;
                match self.matches_impl(original_value, value_pattern, var_pattern, after) {
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
