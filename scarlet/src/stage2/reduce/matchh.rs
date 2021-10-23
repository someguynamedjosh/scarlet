use super::substitute::Substitutions;
use crate::stage2::structure::{BuiltinOperation, BuiltinValue, Definition, Environment, ItemId};

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
    pub(super) fn matches(
        &mut self,
        original_value: ItemId<'x>,
        value_pattern: ItemId<'x>,
        match_against: ItemId<'x>,
    ) -> MatchResult<'x> {
        if let Definition::Variable(var) = self.definition_of(value_pattern) {
            let var_pattern = self.vars[*var].pattern;
            return self.matches(original_value, var_pattern, match_against);
        }
        match self.definition_of(match_against) {
            Definition::BuiltinOperation(op, _) => match op {
                BuiltinOperation::Matches | BuiltinOperation::Dif32U | BuiltinOperation::Sum32U => {
                    Unknown
                }
                BuiltinOperation::_32UPattern | &BuiltinOperation::BoolPattern => {
                    let matches = match self.definition_of(value_pattern) {
                        Definition::BuiltinValue(v) => match v {
                            BuiltinValue::Bool(..) => op == &BuiltinOperation::BoolPattern,
                            BuiltinValue::_32U(..) => op == &BuiltinOperation::_32UPattern,
                            BuiltinValue::GodPattern => false,
                        },
                        Definition::BuiltinOperation(other_op, _) => other_op == op,
                        _ => return Unknown,
                    };
                    if matches {
                        non_capturing_match()
                    } else {
                        NoMatch
                    }
                }
            },
            Definition::BuiltinValue(pv) => match self.definition_of(value_pattern) {
                Definition::BuiltinValue(vv) => {
                    if pv == vv {
                        non_capturing_match()
                    } else {
                        NoMatch
                    }
                }
                Definition::Struct(..) => NoMatch,
                _ => Unknown,
            },
            Definition::Match { .. } => Unknown,
            Definition::Member(_, _) => Unknown,
            Definition::Other(..) => unreachable!(),
            Definition::Struct(_) => todo!(),
            Definition::Substitute(..) => Unknown,
            Definition::Variable(var) => {
                let var = *var;
                let var_pattern = self.vars[var].pattern;
                match self.matches(original_value, value_pattern, var_pattern) {
                    Match(..) => {
                        let mut subs = Substitutions::new();
                        subs.insert_no_replace(var, original_value);
                        Match(subs)
                    }
                    other => other,
                }
            }
        }
    }
}
