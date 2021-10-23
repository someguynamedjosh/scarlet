use super::substitute::Substitutions;
use crate::{
    shared::OrderedSet,
    stage2::structure::{
        BuiltinOperation, BuiltinValue, Definition, Environment, ItemId, VariableId,
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
    pub(super) fn matches(
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
            Definition::BuiltinValue(BuiltinValue::GodPattern) => non_capturing_match(),
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
