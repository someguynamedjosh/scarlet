use itertools::Itertools;

use super::structure::Substitutions;
use crate::{
    stage1::structure::Token,
    stage2::structure::{
        BuiltinOperation, BuiltinValue, Definition, Environment, ItemId, VarType, VariableId,
    },
};

#[derive(Clone, Debug)]
pub enum MatchResult<'x> {
    Match(Substitutions<'x>),
    NoMatch,
    Unknown,
}

use MatchResult::*;

impl<'x> MatchResult<'x> {
    pub fn with_sub_if_match(mut self, target: VariableId<'x>, value: ItemId<'x>) -> Self {
        if let Self::Match(subs) = &mut self {
            subs.insert_no_replace(target, value)
        }
        self
    }
}

fn non_capturing_match<'x>() -> MatchResult<'x> {
    Match(Substitutions::new())
}

fn result_and<'x>(results: Vec<MatchResult<'x>>) -> MatchResult<'x> {
    let mut subs = Substitutions::new();
    let mut unknown = false;
    for result in results {
        match result {
            Match(result_subs) => {
                for (target, value) in result_subs {
                    if let Some(&existing_value) = subs.get(&target) {
                        if value != existing_value {
                            return NoMatch;
                        }
                    } else {
                        subs.insert_no_replace(target, value);
                    }
                }
            }
            NoMatch => return NoMatch,
            Unknown => unknown = true,
        }
    }
    if unknown {
        Unknown
    } else {
        Match(subs)
    }
}

fn result_or<'x>(results: Vec<MatchResult<'x>>) -> MatchResult<'x> {
    let mut unknown = false;
    for result in results {
        match result {
            Match(..) => return result,
            NoMatch => (),
            Unknown => unknown = true,
        }
    }
    if unknown {
        Unknown
    } else {
        NoMatch
    }
}

impl<'x> Environment<'x> {
    pub(super) fn matches(
        &mut self,
        item: ItemId<'x>,
        match_against: ItemId<'x>,
    ) -> MatchResult<'x> {
        let item_bp = self.find_bounding_pattern(item);
        let match_against_bp = self.find_bounding_pattern(match_against);
        self.matches_impl(item, item_bp, match_against_bp, &[])
    }

    fn find_bounding_pattern(&mut self, pattern: ItemId<'x>) -> ItemId<'x> {
        match self.get_definition(pattern).clone() {
            Definition::BuiltinOperation(op, _) => match op {
                BuiltinOperation::Sum32U | BuiltinOperation::Dif32U => todo!(),
            },
            Definition::BuiltinValue(..) => pattern,
            Definition::Match {
                conditions,
                else_value,
                ..
            } => {
                let mut result = else_value;
                for condition in conditions {
                    let valtype = self.find_bounding_pattern(condition.value);
                    result = self.push_var(VarType::Or(valtype, result));
                }
                result
            }
            Definition::Member(..) => todo!(),
            Definition::Other(other) => self.find_bounding_pattern(other),
            Definition::ResolvedSubstitute(base, subs) => todo!(),
            Definition::SetEager { base, vals, eager } => {
                let base = self.find_bounding_pattern(base);
                let def = Definition::SetEager { base, vals, eager };
                self.item_with_new_definition(pattern, def, true)
            }
            Definition::Struct(..) => todo!(),
            Definition::UnresolvedSubstitute(..) => {
                self.resolve_substitution(pattern);
                self.find_bounding_pattern(pattern)
            }
            Definition::Variable { typee, var } => {
                // TODO: Make a function to map a var type.
                let typee = match typee {
                    VarType::God | VarType::_32U | VarType::Bool => typee,
                    VarType::Just(other) => VarType::Just(self.find_bounding_pattern(other)),
                    VarType::And(l, r) => {
                        VarType::And(self.find_bounding_pattern(l), self.find_bounding_pattern(r))
                    }
                    VarType::Or(l, r) => {
                        VarType::Or(self.find_bounding_pattern(l), self.find_bounding_pattern(r))
                    }
                };
                let def = Definition::Variable { typee, var };
                self.item_with_new_definition(pattern, def, true)
            }
        }
    }

    fn matches_impl(
        &mut self,
        original_value: ItemId<'x>,
        value: ItemId<'x>,
        pattern: ItemId<'x>,
        eager_vars: &[VariableId<'x>],
    ) -> MatchResult<'x> {
        let value_def = self.get_definition(value).clone();
        let pattern_def = self.get_definition(pattern).clone();

        //
        // - on right AND,             recurse then and
        // - on left OR,               recurse then and
        // - on right OR,              recurse then or
        // - on left AND,              recurse then or
        // - on right shy variable,    recurse on pattern keeping subs that are deps
        // - on right set_eager,       recurse on base noting eager deps
        // - on left variable,         recurse on pattern
        // - on left and right struct, check fields are same, recurse on fields, then
        //   and
        // - on landr builtin value,   check equal
        // - on landr builtin vartype, check equal
        // - on l bltn val, r bltn vt, check match
        // - otherwise, Unknown

        if let Definition::Variable { var, .. } = pattern_def {
            if eager_vars.contains(&var) {
                return Unknown;
            }
        }
        if let Definition::Variable {
            typee: VarType::And(left, right),
            var,
        } = pattern_def
        {
            let left = self.matches_impl(original_value, value, left, eager_vars);
            let right = self.matches_impl(original_value, value, right, eager_vars);
            result_and(vec![left, right]).with_sub_if_match(var, original_value)
        } else if let Definition::Variable {
            typee: VarType::Or(left, right),
            var,
            ..
        } = value_def
        {
            let left = self.matches_impl(original_value, left, pattern, eager_vars);
            let right = self.matches_impl(original_value, right, pattern, eager_vars);
            result_and(vec![left, right]).with_sub_if_match(var, original_value)
        } else if let Definition::Variable {
            typee: VarType::Or(left, right),
            var,
        } = pattern_def
        {
            let left = self.matches_impl(original_value, value, left, eager_vars);
            let right = self.matches_impl(original_value, value, right, eager_vars);
            result_or(vec![left, right]).with_sub_if_match(var, original_value)
        } else if let Definition::Variable {
            typee: VarType::And(left, right),
            var,
        } = value_def
        {
            let left = self.matches_impl(original_value, left, pattern, eager_vars);
            let right = self.matches_impl(original_value, right, pattern, eager_vars);
            result_or(vec![left, right]).with_sub_if_match(var, original_value)
        } else if let Definition::Variable {
            typee: VarType::Just(other),
            var,
        } = pattern_def
        {
            let result = self.matches_impl(original_value, value, other, eager_vars);
            if let MatchResult::Match(other_subs) = result {
                let mut subs = Substitutions::new();
                let deps = self.get_deps(pattern);
                for (dep, _) in deps {
                    if let Some(sub) = other_subs.get(&dep.var) {
                        subs.insert_no_replace(dep.var, *sub);
                    }
                }
                subs.insert_no_replace(var, original_value);
                MatchResult::Match(subs)
            } else {
                result
            }
        } else if let Definition::Variable {
            typee: VarType::Just(other),
            ..
        } = value_def
        {
            self.matches_impl(original_value, other, pattern, eager_vars)
        } else if let Definition::SetEager { base, vals, eager } = pattern_def {
            if eager {
                let mut new_eagers = eager_vars.to_owned();
                for val in vals {
                    for (dep, _) in self.get_deps(val) {
                        new_eagers.push(dep.var);
                    }
                }
                self.matches_impl(original_value, value, base, &new_eagers[..])
            } else {
                todo!()
            }
        } else if let Definition::SetEager { base, .. } = value_def {
            self.matches_impl(original_value, base, pattern, eager_vars)
        } else if let (Definition::Struct(..), Definition::Struct(..)) = (&value_def, &pattern_def)
        {
            todo!()
        } else if let (&Definition::BuiltinValue(value), &Definition::BuiltinValue(pattern)) =
            (&value_def, &pattern_def)
        {
            if value == pattern {
                non_capturing_match()
            } else {
                NoMatch
            }
        } else if let (
            &Definition::Variable {
                typee: value_type, ..
            },
            &Definition::Variable {
                typee: pattern_type,
                var,
            },
        ) = (&value_def, &pattern_def)
        {
            if value_type == pattern_type {
                non_capturing_match().with_sub_if_match(var, original_value)
            } else {
                NoMatch
            }
        } else if let (&Definition::BuiltinValue(bvalue), &Definition::Variable { typee, var }) =
            (&value_def, &pattern_def)
        {
            let matches = match bvalue {
                BuiltinValue::_32U(..) => typee == VarType::_32U || typee == VarType::God,
                BuiltinValue::Bool(..) => typee == VarType::Bool || typee == VarType::God,
            };
            if matches {
                non_capturing_match().with_sub_if_match(var, original_value)
            } else {
                NoMatch
            }
        } else {
            Unknown
        }
    }
}
