use super::structure::Substitutions;
use crate::stage2::structure::{
    BuiltinOperation, BuiltinValue, Definition, Environment, ItemId, VarType, VariableId,
};

#[derive(Clone, Debug)]
pub enum MatchResult<'x> {
    Match(Substitutions<'x>),
    NoMatch,
    Unknown,
}

use itertools::Itertools;
use MatchResult::*;

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

impl<'x> Environment<'x> {
    pub(super) fn matches(
        &mut self,
        item: ItemId<'x>,
        match_against: ItemId<'x>,
    ) -> MatchResult<'x> {
        self.find_bp_then_match(item, item, match_against)
    }

    fn find_bp_then_match(
        &mut self,
        item: ItemId<'x>,
        // A pattern which $item will always match.
        item_bounding_pattern: ItemId<'x>,
        match_against: ItemId<'x>,
    ) -> MatchResult<'x> {
        match self.get_definition(item).clone() {
            Definition::Match {
                conditions,
                else_value,
                ..
            } => {
                let mut results = conditions
                    .into_iter()
                    .map(|x| self.find_bp_then_match(item, x.value, match_against))
                    .collect_vec();
                results.push(self.find_bp_then_match(item, else_value, match_against));
                return result_and(results);
            }
            Definition::Variable { typee, .. } => {
                if let VarType::Just(other) = typee {
                    return self.find_bp_then_match(item, other, match_against);
                }
            }
            _ => (),
        }
        self.value_with_bp_matches(item, item_bounding_pattern, match_against, &[])
    }

    fn value_with_bp_matches(
        &mut self,
        item: ItemId<'x>,
        item_bounding_pattern: ItemId<'x>,
        match_against: ItemId<'x>,
        eager_vars: &[VariableId<'x>],
    ) -> MatchResult<'x> {
        match self.get_definition(match_against).clone() {
            Definition::BuiltinOperation(_, _) => Unknown,
            Definition::BuiltinValue(pval) => {
                if let &Definition::BuiltinValue(ival) = self.get_definition(item_bounding_pattern)
                {
                    if pval == ival {
                        non_capturing_match()
                    } else {
                        NoMatch
                    }
                } else {
                    Unknown
                }
            }
            Definition::Match {
                base: _,
                conditions: _,
                else_value: _,
            } => todo!(),
            Definition::Member(_, _) => Unknown,
            Definition::Other(match_against) => {
                self.value_with_bp_matches(item, item_bounding_pattern, match_against, eager_vars)
            }
            Definition::SetEager { base, vals, eager } => {
                let vars: Vec<_> = vals
                    .into_iter()
                    .map(|val| self.get_deps(val).into_iter())
                    .flatten()
                    .map(|x| x.0.var)
                    .collect();
                let new_eagers = if eager {
                    [eager_vars.to_owned(), vars].concat()
                } else {
                    eager_vars
                        .iter()
                        .copied()
                        .filter(|x| !vars.contains(x))
                        .collect()
                };
                self.value_with_bp_matches(item, item_bounding_pattern, base, &new_eagers[..])
            }
            Definition::Struct(_) => todo!(),
            Definition::UnresolvedSubstitute(_, _) => todo!(),
            Definition::ResolvedSubstitute(_, _) => todo!(),
            Definition::Variable { var, typee } => {
                let result = self.matches_var_type(item, item_bounding_pattern, typee, eager_vars);
                if let Match(mut subs) = result {
                    subs.insert_no_replace(var, item);
                    let var_deps = self
                        .get_deps(match_against)
                        .into_iter()
                        .map(|x| x.0.var)
                        .collect_vec();
                    let subs = subs
                        .into_iter()
                        .filter(|x| var_deps.contains(&x.0))
                        .collect();
                    Match(subs)
                } else {
                    result
                }
            }
        }
    }

    fn matches_var_type(
        &mut self,
        item: ItemId<'x>,
        // A pattern which $item will always match.
        item_bounding_pattern: ItemId<'x>,
        match_against: VarType<'x>,
        eager_vars: &[VariableId<'x>],
    ) -> MatchResult<'x> {
        let matches = match match_against {
            VarType::God => true,
            VarType::_32U => match self.get_definition(item_bounding_pattern).clone() {
                Definition::BuiltinOperation(op, _) => {
                    op == BuiltinOperation::Sum32U || op == BuiltinOperation::Dif32U
                }
                Definition::BuiltinValue(val) => {
                    if let BuiltinValue::_32U(..) = val {
                        true
                    } else {
                        false
                    }
                }
                Definition::Variable { typee, .. } => match typee {
                    VarType::_32U => true,
                    VarType::Bool => false,
                    _ => return Unknown,
                },
                _ => return Unknown,
            },
            VarType::Bool => match self.get_definition(item_bounding_pattern).clone() {
                Definition::BuiltinOperation(.., _) => false,
                Definition::BuiltinValue(val) => {
                    if let BuiltinValue::Bool(..) = val {
                        true
                    } else {
                        false
                    }
                }
                Definition::Variable { typee, .. } => match typee {
                    VarType::Bool => true,
                    VarType::_32U => false,
                    _ => return Unknown,
                },
                _ => return Unknown,
            },
            VarType::Just(match_against) => {
                return self.value_with_bp_matches(
                    item,
                    item_bounding_pattern,
                    match_against,
                    eager_vars,
                )
            }
            VarType::And(l, r) => {
                let l = self.value_with_bp_matches(item, item_bounding_pattern, l, eager_vars);
                let r = self.value_with_bp_matches(item, item_bounding_pattern, r, eager_vars);
                return result_and(vec![l, r]);
            }
        };
        if matches {
            non_capturing_match()
        } else {
            NoMatch
        }
    }
}
