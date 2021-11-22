use std::marker::PhantomData;

use super::{Pattern, PatternMatchResult, PatternMatchSuccess};
use crate::{
    constructs::{downcast_construct, Construct},
    environment::Environment,
    tokens::structure::Token,
};

pub struct PatCaptureConstruct<C: Construct> {
    pub key: &'static str,
    pub pd: PhantomData<&'static C>,
}

impl<C: Construct> Pattern for PatCaptureConstruct<C> {
    fn match_at<'i, 'x>(
        &self,
        env: &mut Environment<'x>,
        stream: &'i [Token<'x>],
        at_index: usize,
    ) -> PatternMatchResult<'i, 'x> {
        if at_index >= stream.len() {
            Err(())
        } else if let &Token::Construct(con_id) = &stream[at_index] {
            let con = env.get_construct(con_id);
            if downcast_construct::<C>(&**con).is_some() {
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
