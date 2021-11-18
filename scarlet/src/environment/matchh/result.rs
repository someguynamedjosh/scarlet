use std::collections::HashMap;

use crate::{
    constructs::{
        substitution::Substitutions,
        variable::{CVariable, VariableId},
        ConstructId,
    },
    shared::OrderedMap,
};

#[derive(Clone, Debug)]
pub enum MatchResult {
    Match(Substitutions),
    NoMatch,
    Unknown,
}

use MatchResult::*;

impl MatchResult {
    pub fn is_guaranteed_match(&self) -> bool {
        match self {
            Self::Match(..) => true,
            _ => false,
        }
    }

    pub fn with_sub_if_match(mut self, target: CVariable, value: ConstructId) -> Self {
        if let Self::Match(subs) = &mut self {
            subs.insert_no_replace(target, value)
        }
        self
    }

    pub fn keeping_only_eager_subs(self) -> Self {
        todo!()
    }

    pub fn non_capturing() -> Self {
        Match(Substitutions::new())
    }

    pub fn and(results: Vec<Self>) -> Self {
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
