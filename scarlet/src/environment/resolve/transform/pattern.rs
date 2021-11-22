mod basic_token;
mod combinator;
mod construct;
mod tuple_patterns;
mod util;

use std::{collections::HashMap, ops::RangeInclusive};

pub use self::{basic_token::*, combinator::*, tuple_patterns::*};
use crate::{environment::Environment, tokens::structure::Token};

pub type Captures<'i, 'x> = HashMap<&'static str, &'i Token<'x>>;
pub struct PatternMatchSuccess<'i, 'x> {
    pub range: RangeInclusive<usize>,
    pub captures: Captures<'i, 'x>,
}
pub type PatternMatchResult<'i, 'x> = Result<PatternMatchSuccess<'i, 'x>, ()>;

impl<'i, 'x> PatternMatchSuccess<'i, 'x> {
    pub fn at(at_index: usize) -> Self {
        Self {
            range: at_index..=at_index,
            captures: Captures::default(),
        }
    }

    pub fn at_range(range: RangeInclusive<usize>) -> Self {
        Self {
            range,
            captures: Captures::default(),
        }
    }

    pub fn get_capture(&self, key: &str) -> &Token<'x> {
        self.captures.get(key).unwrap()
    }
}

pub trait Pattern {
    fn match_before<'i, 'x>(
        &self,
        env: &mut Environment<'x>,
        stream: &'i [Token<'x>],
        before_index: usize,
    ) -> PatternMatchResult<'i, 'x>;
    fn match_at<'i, 'x>(
        &self,
        env: &mut Environment<'x>,
        stream: &'i [Token<'x>],
        at_index: usize,
    ) -> PatternMatchResult<'i, 'x>;
}

impl<P: Pattern> Pattern for &P {
    fn match_at<'i, 'x>(
        &self,
        env: &mut Environment<'x>,
        stream: &'i [Token<'x>],
        at_index: usize,
    ) -> PatternMatchResult<'i, 'x> {
        (*self).match_at(env, stream, at_index)
    }

    fn match_before<'i, 'x>(
        &self,
        env: &mut Environment<'x>,
        stream: &'i [Token<'x>],
        before_index: usize,
    ) -> PatternMatchResult<'i, 'x> {
        (*self).match_before(env, stream, before_index)
    }
}
