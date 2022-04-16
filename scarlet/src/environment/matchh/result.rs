use crate::item::{substitution::Substitutions, variable::CVariable, ConstructId};

#[derive(Clone, Debug)]
pub enum MatchResult {
    Match,
    NoMatch,
    Unknown,
}

use MatchResult::*;

impl MatchResult {
    pub fn is_guaranteed_match(&self) -> bool {
        match self {
            Self::Match => true,
            _ => false,
        }
    }

    pub fn with_sub_if_match(mut self, target: CVariable, value: ConstructId) -> Self {
        self
    }

    pub fn keeping_only_eager_subs(self) -> Self {
        todo!()
    }

    pub fn non_capturing() -> Self {
        Match
    }

    pub fn and(results: Vec<Self>) -> Self {
        let mut unknown = false;
        for result in results {
            match result {
                Match => (),
                NoMatch => return NoMatch,
                Unknown => unknown = true,
            }
        }
        if unknown {
            Unknown
        } else {
            Match
        }
    }

    pub fn or(results: Vec<Self>) -> Self {
        let mut unknown = false;
        for result in results {
            match result {
                Match => return result,
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
