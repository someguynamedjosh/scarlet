use itertools::Itertools;

use super::Ecc;
use crate::{
    item::{definitions::substitution::Substitutions, resolvable::UnresolvedItemError},
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
            if other
                .get_trimmed_equality(&value)
                .as_ref()
                .map(Equal::is_trivial_yes)
                == Ok(true)
            {
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

    pub fn yes1(left: Substitutions, right: Substitutions) -> Self {
        Self::Yes(left, right)
    }

    pub fn is_trivial_yes(&self) -> bool {
        if let Self::Yes(left, right) = self {
            left.len() == 0 && right.len() == 0
        } else {
            false
        }
    }

    pub fn and(over: Vec<Self>) -> Self {
        let mut default = Self::yes();
        for b in over {
            match b {
                Self::Yes(left, right) => {
                    if let Self::Yes(exleft, exright) = &mut default {
                        let success = combine_substitutions(left, exleft);
                        let success = success.and(combine_substitutions(right, exright));
                        if !success.is_ok() {
                            default = Self::Unknown;
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

    pub fn is_yes(&self) -> bool {
        matches!(self, Self::Yes(..))
    }

    /// Removes substitutions that substitute targets the lhs or rhs is not
    /// dependent on.
    pub(crate) fn filter(&mut self, ctx: &Ecc) -> Result<(), UnresolvedItemError> {
        if let Self::Yes(lhs_subs, rhs_subs) = self {
            let lhs_deps = ctx
                .lhs()
                .get_dependencies()
                .as_complete_variables()?
                .map(|d| d.var.ptr_clone())
                .collect_vec();
            for (target, _) in lhs_subs.clone() {
                if !lhs_deps.contains(&target) {
                    lhs_subs.remove(&target);
                }
            }
            let rhs_deps = ctx
                .rhs()
                .get_dependencies()
                .as_complete_variables()?
                .map(|d| d.var.ptr_clone())
                .collect_vec();
            for (target, _) in rhs_subs.clone() {
                if !rhs_deps.contains(&target) {
                    rhs_subs.remove(&target);
                }
            }
        }
        Ok(())
    }
}
