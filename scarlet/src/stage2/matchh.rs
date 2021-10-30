use std::{hash::BuildHasher, marker::PhantomData};

use super::structure::{Item, Substitutions, VariableId, VariableInfo};
use crate::{
    shared::OrderedSet,
    stage1::structure::TokenTree,
    stage2::structure::{
        BuiltinOperation, BuiltinValue, Definition, Environment, ItemId, Pattern, Variable,
    },
};

#[derive(Clone, Debug)]
pub enum MatchResult<'x> {
    Match(Substitutions<'x>),
    NoMatch,
    Unknown,
}

use serde::de::Unexpected;
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
        self.matches_impl(original_value, match_against)
    }

    pub fn matches_var(
        &mut self,
        original_value: ItemId<'x>,
        var: VariableInfo<'x>,
    ) -> MatchResult<'x> {
        self.matches_var_impl(original_value, original_value, var)
    }

    fn matches_var_impl(
        &mut self,
        original_value: ItemId<'x>,
        value_pattern: ItemId<'x>,
        var: VariableInfo<'x>,
    ) -> MatchResult<'x> {
        todo!()
        // let mut allow_binding = var.consume;
        // match self.matches_impl(original_value, value_pattern, var.typee) {
        //     Match(..) | MatchWithUnknownSubs => {
        //         if allow_binding {
        //             let mut subs = Substitutions::new();
        //             subs.insert_no_replace(var, original_value);
        //             Match(subs)
        //         } else {
        //             Unknown
        //         }
        //     }
        //     other => other,
        // }
    }

    fn get_or_push_pattern(&mut self, expected_pattern: Pattern<'x>) -> ItemId<'x> {
        for (id, item) in &self.items {
            if let Definition::Pattern(pattern) = item.definition.as_ref().unwrap() {
                if pattern == &expected_pattern {
                    return id;
                }
            }
        }
        let var = self.vars.push(Variable { pd: PhantomData });
        let typee = expected_pattern;
        self.items.push(Item {
            original_definition: &TokenTree::Token("INTERNAL"),
            definition: Some(Definition::Pattern(expected_pattern)),
            scope: Default::default(),
            dependencies: None,
            after: None,
            cached_reduction: None,
            shown_from: vec![],
        })
    }

    /// Returns a more vague pattern than the one given.
    fn pattern_parent(&mut self, pattern: ItemId<'x>) -> ItemId<'x> {
        match self.definition_of(pattern).clone() {
            Definition::BuiltinOperation(_, _) => todo!(),
            Definition::BuiltinValue(val) => match val {
                BuiltinValue::_32U(..) => self.get_or_push_pattern(Pattern::_32U),
                BuiltinValue::Bool(..) => self.get_or_push_pattern(Pattern::Bool),
            },
            Definition::Match {
                base,
                conditions,
                else_value,
            } => unreachable!(),
            Definition::Member(_, _) => todo!(),
            Definition::Other(_) => todo!(),
            Definition::Pattern(..) => self.get_or_push_pattern(Pattern::Pattern),
            Definition::Struct(_) => todo!(),
            Definition::UnresolvedSubstitute(_, _) => todo!(),
            Definition::ResolvedSubstitute(_, _) => todo!(),
            Definition::Variable { pattern, .. } => self.pattern_parent(pattern),
        }
    }

    fn common_parent_of_two_patterns(
        &mut self,
        pattern0: ItemId<'x>,
        pattern1: ItemId<'x>,
    ) -> ItemId<'x> {
        if pattern0 == pattern1 {
            pattern0
        } else {
            let mut pattern1_parent = self.pattern_parent(pattern1);
            loop {
                if pattern0 == pattern1_parent {
                    return pattern0;
                }
                let next = self.pattern_parent(pattern1_parent);
                if next == pattern1_parent {
                    break;
                } else {
                    pattern1_parent = next;
                }
            }
            let parent_of_pattern0 = self.pattern_parent(pattern0);
            self.common_parent_of_two_patterns(parent_of_pattern0, pattern1)
        }
    }

    fn common_parent_of_patterns(&mut self, patterns: &[ItemId<'x>]) -> ItemId<'x> {
        assert!(patterns.len() > 0);
        if patterns.len() == 1 {
            patterns[0]
        } else {
            let common_parent_of_remainder = self.common_parent_of_patterns(&patterns[1..]);
            self.common_parent_of_two_patterns(patterns[0], common_parent_of_remainder)
        }
    }

    pub fn as_pattern(&mut self, of: ItemId<'x>) -> ItemId<'x> {
        let def = self.definition_of(of);
        match def {
            Definition::BuiltinOperation(op, _) => match op {
                BuiltinOperation::Sum32U | BuiltinOperation::Dif32U => {
                    self.get_or_push_pattern(Pattern::_32U)
                }
            },
            Definition::BuiltinValue(..) => of,
            Definition::Match {
                conditions,
                else_value,
                ..
            } => {
                let else_value = *else_value;
                let conditions = conditions.clone();
                let else_pattern = self.as_pattern(else_value);
                let patterns: Vec<_> = conditions
                    .iter()
                    .map(|c| self.as_pattern(c.value))
                    .chain(std::iter::once(else_pattern))
                    .collect();
                self.common_parent_of_patterns(&patterns[..])
            }
            Definition::Member(_, _) => todo!(),
            Definition::Other(_) => todo!(),
            Definition::Pattern(..) => of,
            Definition::Struct(_) => todo!(),
            Definition::UnresolvedSubstitute(_, _) => todo!(),
            Definition::ResolvedSubstitute(_, _) => todo!(),
            &Definition::Variable { var, pattern } => {
                let pattern = self.as_pattern(pattern);
                let var = VariableInfo {
                    var_item: of,
                    var,
                    pattern,
                };
                let captured = Definition::Pattern(Pattern::Capture(var));
                self.item_with_new_definition(of, captured, true)
            }
        }
    }

    fn matches_impl(
        &mut self,
        original_value: ItemId<'x>,
        match_against: ItemId<'x>,
    ) -> MatchResult<'x> {
        let value_pattern = self.as_pattern(original_value);
        self.matches_def(original_value, value_pattern, match_against)
    }

    fn matches_def(
        &mut self,
        original_value: ItemId<'x>,
        value_pattern: ItemId<'x>,
        match_against: ItemId<'x>,
    ) -> MatchResult<'x> {
        println!("{:#?}", self);
        match self.definition_of(match_against).clone() {
            Definition::BuiltinOperation(op, _) => match op {
                BuiltinOperation::Dif32U | BuiltinOperation::Sum32U => Unknown,
            },
            Definition::BuiltinValue(pv) => match self.definition_of(value_pattern) {
                Definition::BuiltinValue(vv) => {
                    if pv == *vv {
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
            Definition::Member(..) => Unknown,
            Definition::Other(other) => self.matches_def(original_value, value_pattern, other),
            Definition::Pattern(pat) => self.matches_pattern(original_value, value_pattern, pat),
            Definition::ResolvedSubstitute(..) => Unknown,
            Definition::Struct(_) => todo!(),
            Definition::UnresolvedSubstitute(..) => Unknown,
            Definition::Variable { var, pattern } => Unknown,
        }
    }

    fn matches_pattern(
        &mut self,
        original_value: ItemId<'x>,
        value_pattern: ItemId<'x>,
        pattern: Pattern<'x>,
    ) -> MatchResult<'x> {
        match pattern {
            Pattern::God => non_capturing_match(),
            Pattern::Pattern => non_capturing_match(),
            Pattern::_32U => match self.definition_of(value_pattern) {
                Definition::BuiltinValue(val) => {
                    if let BuiltinValue::_32U(..) = val {
                        non_capturing_match()
                    } else {
                        NoMatch
                    }
                }
                Definition::Pattern(Pattern::_32U) => non_capturing_match(),
                Definition::Pattern(_) => non_capturing_match(),
                Definition::Struct(_) => NoMatch,
                Definition::Variable { .. } => todo!(),
                _ => unreachable!(),
            },
            Pattern::Bool => match self.definition_of(value_pattern) {
                Definition::BuiltinValue(val) => {
                    if let BuiltinValue::Bool(..) = val {
                        non_capturing_match()
                    } else {
                        NoMatch
                    }
                }
                Definition::Pattern(Pattern::Bool) => non_capturing_match(),
                Definition::Pattern(_) => non_capturing_match(),
                Definition::Struct(_) => NoMatch,
                Definition::Variable { .. } => todo!(),
                _ => unreachable!(),
            },
            Pattern::Capture(var) => {
                let result = self.matches_def(original_value, value_pattern, var.pattern);
                match result {
                    Match(subs) => {
                        let mut subs = subs;
                        subs.insert_no_replace(var.var, original_value);
                        Match(subs)
                    }
                    NoMatch | Unknown => result,
                }
            }
            Pattern::And(left, right) => {
                let matches_left = self.matches_def(original_value, value_pattern, left);
                let matches_right = self.matches_def(original_value, value_pattern, right);
                match (matches_left, matches_right) {
                    (Match(left), Match(right)) => Match(left.union(right)),
                    (NoMatch, _) | (_, NoMatch) => NoMatch,
                    _ => Unknown,
                }
            }
        }
    }
}
