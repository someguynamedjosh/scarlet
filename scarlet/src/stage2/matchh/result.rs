use crate::stage2::{
    construct::constructs::Substitutions,
    structure::{ConstructId, VariableId},
};

#[derive(Clone, Debug)]
pub enum MatchResult<'x> {
    Match(Substitutions<'x>),
    NoMatch,
    Unknown,
}

use MatchResult::*;

impl<'x> MatchResult<'x> {
    pub fn is_guaranteed_match(&self) -> bool {
        match self {
            Self::Match(..) => true,
            _ => false,
        }
    }

    pub fn with_sub_if_match(mut self, target: VariableId<'x>, value: ConstructId<'x>) -> Self {
        if let Self::Match(subs) = &mut self {
            subs.insert_no_replace(target, value)
        }
        self
    }

    pub fn keeping_only_eager_subs(self, eager_vars: &[VariableId<'x>]) -> Self {
        match self {
            Self::Match(subs) => {
                let mut new_subs = crate::stage2::construct::constructs::Substitutions::new();
                for sub in subs {
                    if eager_vars.contains(&sub.0) {
                        new_subs.insert_no_replace(sub.0, sub.1);
                    }
                }
                Self::Match(new_subs)
            }
            other => other,
        }
    }

    pub fn non_capturing() -> Self {
        Match(crate::stage2::construct::constructs::Substitutions::new())
    }

    pub fn and(results: Vec<Self>) -> Self {
        let mut subs = crate::stage2::construct::constructs::Substitutions::new();
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

    pub fn or(results: Vec<Self>) -> Self {
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
}
