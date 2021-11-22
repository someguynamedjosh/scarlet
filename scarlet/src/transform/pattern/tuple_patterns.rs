use super::{util, Pattern, PatternMatchResult};
use crate::{environment::Environment, tokens::structure::Token};

impl<P0: Pattern> Pattern for (P0,) {
    fn match_at<'i, 'x>(
        &self,
        env: &mut Environment<'x>,
        stream: &'i [Token<'x>],
        at_index: usize,
    ) -> PatternMatchResult<'i, 'x> {
        self.0.match_at(env, stream, at_index)
    }

    fn match_before<'i, 'x>(
        &self,
        env: &mut Environment<'x>,
        stream: &'i [Token<'x>],
        before_index: usize,
    ) -> PatternMatchResult<'i, 'x> {
        self.0.match_before(env, stream, before_index)
    }
}

impl<P0: Pattern, P1: Pattern> Pattern for (P0, P1) {
    fn match_at<'i, 'x>(
        &self,
        env: &mut Environment<'x>,
        stream: &'i [Token<'x>],
        at_index: usize,
    ) -> PatternMatchResult<'i, 'x> {
        let base = self.0.match_at(env, stream, at_index)?;
        let next = self.1.match_at(env, stream, *base.range.end() + 1)?;
        Ok(util::pms_union(vec![base, next]))
    }

    fn match_before<'i, 'x>(
        &self,
        env: &mut Environment<'x>,
        stream: &'i [Token<'x>],
        before_index: usize,
    ) -> PatternMatchResult<'i, 'x> {
        let base = self.0.match_before(env, stream, before_index)?;
        let previous = self.1.match_before(env, stream, *base.range.start())?;
        Ok(util::pms_union(vec![previous, base]))
    }
}

macro_rules! three_or_more_tuple_impl {
    ($($TemplateParam:ident $field:tt,)*) => {
        impl<P0: Pattern, $($TemplateParam : Pattern),*> Pattern for (P0, $($TemplateParam),*) {
            fn match_at<'i, 'x>(
                &self,
                env: &mut Environment<'x>,
                stream: &'i [Token<'x>],
                at_index: usize,
            ) -> PatternMatchResult<'i, 'x> {
                (&self.0, ($(&self.$field),*)).match_at(env, stream, at_index)
            }

            fn match_before<'i, 'x>(
                &self,
                env: &mut Environment<'x>,
                stream: &'i [Token<'x>],
                before_index: usize,
            ) -> PatternMatchResult<'i, 'x> {
                (&self.0, ($(&self.$field),*)).match_before(env, stream, before_index)
            }
        }
    };
}

three_or_more_tuple_impl!(P1 1, P2 2, );
three_or_more_tuple_impl!(P1 1, P2 2, P3 3, );
three_or_more_tuple_impl!(P1 1, P2 2, P3 3, P4 4, );
three_or_more_tuple_impl!(P1 1, P2 2, P3 3, P4 4, P5 5, );
three_or_more_tuple_impl!(P1 1, P2 2, P3 3, P4 4, P5 5, P6 6, );
