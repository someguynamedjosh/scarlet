use super::{util, Pattern, PatternMatchResult};
use crate::{environment::Environment, tokens::structure::Token};

#[derive(Debug)]
pub struct PatPreceded<Before: Pattern, At: Pattern>(Before, At);

impl<Before: Pattern, At: Pattern> Pattern for PatPreceded<Before, At> {
    fn match_at<'i, 'x>(
        &self,
        env: &mut Environment<'x>,
        stream: &'i [Token<'x>],
        at_index: usize,
    ) -> PatternMatchResult<'i, 'x> {
        Ok(util::pms_union(vec![
            self.0.match_before(env, stream, at_index)?,
            self.1.match_at(env, stream, at_index)?,
        ]))
    }

    fn match_before<'i, 'x>(
        &self,
        env: &mut Environment<'x>,
        stream: &'i [Token<'x>],
        before_index: usize,
    ) -> PatternMatchResult<'i, 'x> {
        Ok(util::pms_union(vec![
            self.0.match_before(env, stream, before_index)?,
            self.1.match_at(env, stream, before_index)?,
        ]))
    }
}

#[derive(Debug)]
pub struct PatFirstOf(pub Vec<Box<dyn Pattern>>);

impl Pattern for PatFirstOf {
    fn match_at<'i, 'x>(
        &self,
        env: &mut Environment<'x>,
        stream: &'i [Token<'x>],
        at_index: usize,
    ) -> PatternMatchResult<'i, 'x> {
        for pat in &self.0 {
            if let Ok(result) = pat.match_at(env, stream, at_index) {
                return Ok(result);
            }
        }
        Err(())
    }

    fn match_before<'i, 'x>(
        &self,
        env: &mut Environment<'x>,
        stream: &'i [Token<'x>],
        before_index: usize,
    ) -> PatternMatchResult<'i, 'x> {
        for pat in &self.0 {
            if let Ok(result) = pat.match_before(env, stream, before_index) {
                return Ok(result);
            }
        }
        Err(())
    }
}
