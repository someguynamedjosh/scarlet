use std::{hash::BuildHasher, marker::PhantomData};

use super::structure::{Item, Substitutions, VariableId, VariableInfo};
use crate::{
    shared::OrderedSet,
    stage1::structure::TokenTree,
    stage2::structure::{
        BuiltinOperation, BuiltinValue, Definition, Environment, ItemId, VarType, Variable,
    },
};

#[derive(Clone, Debug)]
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
        self.matches_impl(original_value, original_value, match_against)
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

    fn get_or_push_var_with_type(&mut self, expected_typee: VarType<'x>) -> ItemId<'x> {
        for (id, item) in &self.items {
            if let Definition::Variable { typee, .. } = item.definition.as_ref().unwrap() {
                if typee == &expected_typee {
                    return id;
                }
            }
        }
        let var = self.vars.push(Variable { pd: PhantomData });
        let typee = expected_typee;
        self.items.push(Item {
            original_definition: &TokenTree::Token("INTERNAL"),
            definition: Some(Definition::Variable { typee, var }),
            scope: Default::default(),
            dependencies: None,
            after: None,
            cached_reduction: None,
            shown_from: vec![],
        })
    }

    /// Returns a more vague pattern than the one given.
    fn parent_of_super_pattern(&mut self, super_pattern: ItemId<'x>) -> ItemId<'x> {
        match self.definition_of(super_pattern) {
            Definition::BuiltinOperation(_, _) => todo!(),
            Definition::BuiltinValue(val) => match val {
                BuiltinValue::_32U(..) => self.get_or_push_var_with_type(VarType::_32U),
                BuiltinValue::Bool(..) => self.get_or_push_var_with_type(VarType::Bool),
            },
            Definition::Match {
                base,
                conditions,
                else_value,
            } => unreachable!(),
            Definition::Member(_, _) => todo!(),
            Definition::Other(_) => todo!(),
            Definition::SetConsume { .. } => todo!(),
            Definition::Struct(_) => todo!(),
            Definition::UnresolvedSubstitute(_, _) => todo!(),
            Definition::ResolvedSubstitute(_, _) => todo!(),
            Definition::Variable { typee: matches, .. } => {
                let matches = *matches;
                // let super_matches = self.as_super_pattern(matches);
                // self.parent_of_super_pattern(super_matches)
                todo!()
            }
        }
    }

    fn common_parent_of_two_super_patterns(
        &mut self,
        pattern0: ItemId<'x>,
        pattern1: ItemId<'x>,
    ) -> ItemId<'x> {
        if pattern0 == pattern1 {
            pattern0
        } else {
            let mut pattern1_parent = self.parent_of_super_pattern(pattern1);
            loop {
                if pattern0 == pattern1_parent {
                    return pattern0;
                }
                let next = self.parent_of_super_pattern(pattern1_parent);
                if next == pattern1_parent {
                    break;
                } else {
                    pattern1_parent = next;
                }
            }
            let parent_of_pattern0 = self.parent_of_super_pattern(pattern0);
            self.common_parent_of_two_super_patterns(parent_of_pattern0, pattern1)
        }
    }

    fn common_parent_of_super_patterns(&mut self, patterns: &[ItemId<'x>]) -> ItemId<'x> {
        assert!(patterns.len() > 0);
        if patterns.len() == 1 {
            patterns[0]
        } else {
            let common_parent_of_remainder = self.common_parent_of_super_patterns(&patterns[1..]);
            self.common_parent_of_two_super_patterns(patterns[0], common_parent_of_remainder)
        }
    }

    fn as_super_pattern(&mut self, of: ItemId<'x>) -> ItemId<'x> {
        let def = self.definition_of(of);
        match def {
            Definition::BuiltinOperation(op, _) => match op {
                BuiltinOperation::Sum32U | BuiltinOperation::Dif32U => {
                    self.get_or_push_var_with_type(VarType::_32U)
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
                let else_pattern = self.as_super_pattern(else_value);
                let patterns: Vec<_> = conditions
                    .iter()
                    .map(|c| self.as_super_pattern(c.value))
                    .chain(std::iter::once(else_pattern))
                    .collect();
                self.common_parent_of_super_patterns(&patterns[..])
            }
            Definition::Member(_, _) => todo!(),
            Definition::Other(_) => todo!(),
            Definition::SetConsume { .. } => todo!(),
            Definition::Struct(_) => todo!(),
            Definition::UnresolvedSubstitute(_, _) => todo!(),
            Definition::ResolvedSubstitute(_, _) => todo!(),
            &Definition::Variable { var, typee } => {
                let super_type = match typee {
                    VarType::God | VarType::_32U | VarType::Bool => typee,
                    VarType::Just(other) => VarType::Just(self.as_super_pattern(other)),
                    VarType::And(l, r) => {
                        VarType::And(self.as_super_pattern(l), self.as_super_pattern(r))
                    }
                };
                let typee = super_type;
                self.item_with_new_definition(of, Definition::Variable { var, typee }, true)
            }
        }
    }

    fn matches_impl(
        &mut self,
        original_value: ItemId<'x>,
        value_pattern: ItemId<'x>,
        match_against: ItemId<'x>,
    ) -> MatchResult<'x> {
        let value_super_pattern = self.as_super_pattern(value_pattern);
        self.matches_def(original_value, value_super_pattern, match_against)
    }

    fn matches_def(
        &mut self,
        original_value: ItemId<'x>,
        value_super_pattern: ItemId<'x>,
        match_against: ItemId<'x>,
    ) -> MatchResult<'x> {
        match self.definition_of(match_against) {
            Definition::BuiltinOperation(op, _) => match op {
                BuiltinOperation::Dif32U | BuiltinOperation::Sum32U => Unknown,
            },
            Definition::BuiltinValue(pv) => match self.definition_of(value_super_pattern) {
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
            Definition::Member(..) => Unknown,
            &Definition::Other(other) => {
                self.matches_def(original_value, value_super_pattern, other)
            }
            Definition::ResolvedSubstitute(..) => Unknown,
            Definition::SetConsume { .. } => todo!(),
            Definition::Struct(_) => todo!(),
            Definition::UnresolvedSubstitute(..) => Unknown,
            &Definition::Variable { var, typee } => {
                self.matches_var_type(original_value, value_super_pattern, var, typee)
            }
        }
    }

    fn matches_var_type(
        &mut self,
        original_value: ItemId<'x>,
        value_super_pattern: ItemId<'x>,
        var: VariableId<'x>,
        typee: VarType<'x>,
    ) -> MatchResult<'x> {
        let result = match typee {
            VarType::God => non_capturing_match(),
            VarType::_32U => match self.definition_of(value_super_pattern) {
                Definition::BuiltinValue(val) => {
                    if let BuiltinValue::_32U(..) = val {
                        non_capturing_match()
                    } else {
                        NoMatch
                    }
                }
                Definition::Struct(_) => NoMatch,
                Definition::Variable { typee, .. } => match typee {
                    VarType::_32U => non_capturing_match(),
                    VarType::Just(_) => todo!(),
                    VarType::And(_, _) => todo!(),
                    _ => NoMatch,
                },
                _ => unreachable!(),
            },
            VarType::Bool => match self.definition_of(value_super_pattern) {
                Definition::BuiltinValue(val) => {
                    if let BuiltinValue::Bool(..) = val {
                        non_capturing_match()
                    } else {
                        NoMatch
                    }
                }
                Definition::Struct(_) => NoMatch,
                Definition::Variable { typee, .. } => match typee {
                    VarType::Bool => non_capturing_match(),
                    VarType::Just(_) => todo!(),
                    VarType::And(_, _) => todo!(),
                    _ => NoMatch,
                },
                _ => unreachable!(),
            },
            VarType::Just(other) => self.matches_def(original_value, value_super_pattern, other),
            VarType::And(_, _) => todo!(),
        };
        match result {
            Match(mut subs) => {
                subs.insert_no_replace(var, original_value);
                Match(subs)
            }
            NoMatch | Unknown => result,
        }
    }
}
