use super::super::{Environment, UnresolvedConstructError};
use crate::constructs::{substitution::Substitutions, ConstructId};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Equal {
    Yes(Substitutions),
    NeedsHigherLimit,
    Unknown,
    No,
}

fn combine_substitutions(
    from: Substitutions,
    target_subs: &mut Substitutions,
) -> Result<(), ()> {
    for (target, value) in from {
        if target_subs.contains_key(&target) {
            return Err(());
        } else {
            target_subs.insert_no_replace(target, value);
        }
    }
    Ok(())
}

impl Equal {
    pub fn yes() -> Self {
        Self::Yes(Default::default())
    }

    pub fn and(over: Vec<Self>) -> Self {
        let mut default = Self::yes();
        for b in over {
            match b {
                Self::Yes(left) => {
                    if let Self::Yes(exleft) = &mut default {
                        let success = (|| -> Result<(), ()> {
                            combine_substitutions(left, exleft)?;
                            Ok(())
                        })();
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

    /// Returns `true` if the equal is [`NeedsHigherLimit`].
    ///
    /// [`NeedsHigherLimit`]: Equal::NeedsHigherLimit
    pub fn is_needs_higher_limit(&self) -> bool {
        matches!(self, Self::NeedsHigherLimit)
    }
}
