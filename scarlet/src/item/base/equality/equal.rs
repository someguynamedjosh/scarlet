use itertools::Itertools;

use crate::{
    item::{
        definitions::{substitution::Substitutions, variable::VariablePtr},
        ItemPtr,
    },
    util::PtrExtension,
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Equal {
    Yes(Substitutions, Substitutions),
    Unknown,
    No,
}

fn combine_substitutions(from: Substitutions, target_subs: &mut Substitutions) -> Result<(), ()> {
    for (target, value) in from {
        let value = value.dereference();
        if let Some(other) = target_subs.get(&target) {
            if other.dereference() == value.dereference() {
                continue;
            } else {
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

    pub fn and(over: Vec<Self>) -> Self {
        let mut default = Self::yes();
        for b in over {
            match b {
                Self::Yes(left, mut right) => {
                    if let Self::Yes(exleft, exright) = &mut default {
                        let success = combine_substitutions(left, exleft);
                        let success = success.and(combine_substitutions(right, exright));
                        if success.is_err() {
                            default = Self::Unknown
                        }
                    }
                }
                Self::Unknown | Self::No => default = Self::Unknown,
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
                Self::No => (),
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

    pub fn reorder(self, order: &[&VariablePtr]) -> Self {
        match self {
            Self::Yes(subs, recursion) => Self::Yes(subs.reorder(order), recursion),
            other => other,
        }
    }

    pub(crate) fn sort(self) -> Equal {
        match self {
            Self::Yes(subs, recursion) => {
                let mut order = subs.iter().map(|(k, _)| k.ptr_clone()).collect_vec();
                // order.sort_by_key(|x| &env.get_variable(*x).order);
                todo!();
                Self::Yes(subs.reorder(&order.iter().collect_vec()), recursion)
            }
            other => other,
        }
    }
}
