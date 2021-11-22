use super::{util, Pattern, PatternMatchResult, PatternMatchSuccess};
use crate::tokens::structure::Token;

pub struct PatPreceded<Before: Pattern, At: Pattern>(Before, At);

impl<Before: Pattern, At: Pattern> Pattern for PatPreceded<Before, At> {
    fn match_at<'i, 'x>(
        &self,
        stream: &'i [Token<'x>],
        at_index: usize,
    ) -> PatternMatchResult<'i, 'x> {
        Ok(util::pms_union(vec![
            self.0.match_before(stream, at_index)?,
            self.1.match_at(stream, at_index)?,
        ]))
    }

    fn match_before<'i, 'x>(
        &self,
        stream: &'i [Token<'x>],
        before_index: usize,
    ) -> PatternMatchResult<'i, 'x> {
        Ok(util::pms_union(vec![
            self.0.match_before(stream, before_index)?,
            self.1.match_at(stream, before_index)?,
        ]))
    }
}

pub struct PatFirstOf(pub Vec<Box<dyn Pattern>>);

impl Pattern for PatFirstOf {
    fn match_at<'i, 'x>(
        &self,
        stream: &'i [Token<'x>],
        at_index: usize,
    ) -> PatternMatchResult<'i, 'x> {
        for pat in &self.0 {
            if let Ok(result) = pat.match_at(stream, at_index) {
                return Ok(result);
            }
        }
        Err(())
    }

    fn match_before<'i, 'x>(
        &self,
        stream: &'i [Token<'x>],
        before_index: usize,
    ) -> PatternMatchResult<'i, 'x> {
        for pat in &self.0 {
            if let Ok(result) = pat.match_before(stream, before_index) {
                return Ok(result);
            }
        }
        Err(())
    }
}
