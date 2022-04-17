use itertools::Itertools;

use crate::item::{
    definitions::{substitution::Substitutions, variable::VariableId},
    ItemPtr,
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Equal {
    Yes(Substitutions, Vec<ItemPtr>),
    NeedsHigherLimit,
    Unknown,
    No,
}

fn combine_substitutions(from: Substitutions, target_subs: &mut Substitutions) -> Result<(), ()> {
    for (target, value) in from {
        if target_subs.contains_key(&target) {
            if target_subs.get(&target) != Some(&value) {
                return Err(());
            }
        } else {
            target_subs.insert_no_replace(target, value);
        }
    }
    Ok(())
}

impl Equal {
    pub fn yes() -> Self {
        Self::Yes(Default::default(), Default::default())
    }

    pub fn yes_recursing_over(recursing_over: Vec<ItemPtr>) -> Self {
        Self::Yes(Default::default(), recursing_over)
    }

    pub fn and(over: Vec<Self>) -> Self {
        let mut default = Self::yes();
        for b in over {
            match b {
                Self::Yes(left, mut lrecursion) => {
                    if let Self::Yes(exleft, exrecursion) = &mut default {
                        let success = combine_substitutions(left, exleft);
                        exrecursion.append(&mut lrecursion);
                        if success.is_err() {
                            default = Self::Unknown
                        }
                    }
                }
                Self::NeedsHigherLimit => {
                    if let Self::Yes(..) = default {
                        default = Self::NeedsHigherLimit
                    }
                }
                Self::Unknown => default = Self::Unknown,
                Self::No => return Self::No,
            }
        }
        default
    }

    pub fn or(over: Vec<Self>) -> Self {
        let mut default = Self::No;
        for b in over {
            match b {
                Self::Yes(..) => return b,
                Self::Unknown => {
                    if let Self::No = default {
                        default = Self::Unknown
                    }
                }
                Self::NeedsHigherLimit => default = Self::NeedsHigherLimit,
                Self::No => return Self::No,
            }
        }
        default
    }

    pub fn without_subs(self) -> Self {
        match self {
            Self::Yes(..) => Self::yes(),
            other => other,
        }
    }

    pub fn reorder(self, order: &[&VariableId]) -> Self {
        match self {
            Self::Yes(subs, recursion) => Self::Yes(subs.reorder(order), recursion),
            other => other,
        }
    }

    /// Returns `true` if the equal is [`NeedsHigherLimit`].
    ///
    /// [`NeedsHigherLimit`]: Equal::NeedsHigherLimit
    pub fn is_needs_higher_limit(&self) -> bool {
        matches!(self, Self::NeedsHigherLimit)
    }

    pub(crate) fn sort(self) -> Equal {
        match self {
            Self::Yes(subs, recursion) => {
                let mut order = subs.iter().map(|(k, _)| *k).collect_vec();
                // order.sort_by_key(|x| &env.get_variable(*x).order);
                todo!();
                Self::Yes(subs.reorder(&order.iter().collect_vec()), recursion)
            }
            other => other,
        }
    }

    pub(crate) fn recursing_over(self, recurses_over: Vec<ItemPtr>) -> Self {
        match self {
            Equal::Yes(subs, previous_recurses_over) => Self::Yes(
                subs,
                recurses_over
                    .into_iter()
                    .chain(previous_recurses_over)
                    .collect(),
            ),
            _ => self,
        }
    }
}
