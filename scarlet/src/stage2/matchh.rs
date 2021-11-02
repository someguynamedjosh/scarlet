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

impl<'x> Environment<'x> {
    pub(super) fn matches(
        &mut self,
        item: ItemId<'x>,
        match_against: ItemId<'x>,
    ) -> MatchResult<'x> {
        self.matches_impl(item, item, match_against, &[])
    }

    fn get_or_push_pattern(&mut self, requested_type: VarType<'x>) -> ItemId<'x> {
        for (id, item) in &self.items {
            if let &Some(Definition::Variable { typee, .. }) = &item.definition {
                if typee == requested_type {
                    return id;
                }
            }
        }
        todo!()
    }

    fn matches_impl(
        &mut self,
        item: ItemId<'x>,
        // A pattern which $item will always match.
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
                self.matches_impl(item, item_bounding_pattern, match_against, eager_vars)
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
                self.matches_impl(item, item_bounding_pattern, base, &new_eagers[..])
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
                    VarType::God => return Unknown,
                    VarType::_32U => true,
                    _ => false,
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
                    VarType::God => return Unknown,
                    VarType::Bool => true,
                    _ => false,
                },
                _ => return Unknown,
            },
            VarType::Just(match_against) => {
                return self.matches_impl(item, item_bounding_pattern, match_against, eager_vars)
            }
            VarType::And(_, _) => todo!(),
        };
        if matches {
            non_capturing_match()
        } else {
            NoMatch
        }
    }
}
