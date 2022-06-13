use itertools::Itertools;

use super::Ecc;
use crate::{
    item::{
        definitions::{substitution::Substitutions, variable::VariablePtr},
        ItemPtr,
    },
    util::PtrExtension,
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Equal {
    Yes(Vec<(Substitutions, Substitutions)>),
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
        Self::Yes(vec![(Default::default(), Default::default())])
    }

    pub fn yes1(left: Substitutions, right: Substitutions) -> Self {
        Self::Yes(vec![(left, right)])
    }

    pub fn is_trivial_yes(&self) -> bool {
        if let Self::Yes(cases) = self {
            cases.iter().any(|x| x.0.len() == 0 && x.1.len() == 0)
        } else {
            false
        }
    }

    pub fn and(over: Vec<Self>) -> Self {
        let mut default = Self::yes();
        for b in over {
            match b {
                Self::Yes(other_subs) => {
                    if let Self::Yes(ex_subs) = std::mem::replace(&mut default, Self::Unknown) {
                        let mut new_subs = Vec::new();
                        for ((left, right), (mut exleft, mut exright)) in other_subs
                            .into_iter()
                            .cartesian_product(ex_subs.into_iter())
                        {
                            let success = combine_substitutions(left, &mut exleft);
                            let success = success.and(combine_substitutions(right, &mut exright));
                            if success.is_ok() {
                                new_subs.push((exleft, exright));
                            }
                        }
                        if new_subs.len() > 0 {
                            default = Self::Yes(new_subs);
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
    pub(crate) fn filter(&mut self, ctx: &Ecc) {
        if let Self::Yes(subs) = self {
            for (lhs_subs, rhs_subs) in subs {
                let lhs_deps = ctx
                    .lhs()
                    .get_dependencies()
                    .as_variables()
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
                    .as_variables()
                    .map(|d| d.var.ptr_clone())
                    .collect_vec();
                for (target, _) in rhs_subs.clone() {
                    if !rhs_deps.contains(&target) {
                        rhs_subs.remove(&target);
                    }
                }
            }
        }
    }
}
