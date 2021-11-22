use super::{Pattern, PatternMatchResult, PatternMatchSuccess};
use crate::{environment::Environment, tokens::structure::Token};

pub struct PatCaptureAny {
    pub key: &'static str,
}

impl Pattern for PatCaptureAny {
    fn match_at<'i, 'x>(
        &self,
        _env: &mut Environment<'x>,
        stream: &'i [Token<'x>],
        at_index: usize,
    ) -> PatternMatchResult<'i, 'x> {
        if at_index >= stream.len() {
            Err(())
        } else {
            let mut res = PatternMatchSuccess::at(at_index);
            res.captures.insert(self.key, &stream[at_index]);
            Ok(res)
        }
    }

    fn match_before<'i, 'x>(
        &self,
        env: &mut Environment<'x>,
        stream: &'i [Token<'x>],
        before_index: usize,
    ) -> PatternMatchResult<'i, 'x> {
        if before_index < 1 {
            Err(())
        } else {
            Self::match_at(&self, env, stream, before_index - 1)
        }
    }
}

pub struct PatCaptureStream {
    pub key: &'static str,
    pub label: &'static str,
}

impl Pattern for PatCaptureStream {
    fn match_at<'i, 'x>(
        &self,
        env: &mut Environment<'x>,
        stream: &'i [Token<'x>],
        at_index: usize,
    ) -> PatternMatchResult<'i, 'x> {
        if at_index >= stream.len() {
            Err(())
        } else if let Token::Stream { label, .. } = &stream[at_index] {
            if *label == self.label {
                let mut res = PatternMatchSuccess::at(at_index);
                res.captures.insert(self.key, &stream[at_index]);
                Ok(res)
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }

    fn match_before<'i, 'x>(
        &self,
        env: &mut Environment<'x>,
        stream: &'i [Token<'x>],
        before_index: usize,
    ) -> PatternMatchResult<'i, 'x> {
        if before_index < 1 {
            Err(())
        } else {
            Self::match_at(&self, env, stream, before_index - 1)
        }
    }
}
pub struct PatPlain(pub &'static str);

impl Pattern for PatPlain {
    fn match_at<'i, 'x>(
        &self,
        env: &mut Environment<'x>,
        stream: &'i [Token<'x>],
        at_index: usize,
    ) -> PatternMatchResult<'i, 'x> {
        if at_index >= stream.len() {
            Err(())
        } else if stream[at_index] == self.0.into() {
            Ok(PatternMatchSuccess::at(at_index))
        } else {
            Err(())
        }
    }

    fn match_before<'i, 'x>(
        &self,
        env: &mut Environment<'x>,
        stream: &'i [Token<'x>],
        before_index: usize,
    ) -> PatternMatchResult<'i, 'x> {
        if before_index < 1 {
            Err(())
        } else {
            Self::match_at(&self, env, stream, before_index - 1)
        }
    }
}
