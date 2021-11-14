use std::{collections::HashMap, marker::PhantomData, ops::RangeInclusive};

use crate::stage2::structure::Token;

pub type Captures<'x> = HashMap<&'static str, &'x Token<'x>>;
pub struct PatternMatchSuccess<'x> {
    range: RangeInclusive<usize>,
    captures: Captures<'x>,
}
pub type PatternMatchResult<'x> = Result<PatternMatchSuccess<'x>, ()>;

impl<'x> PatternMatchSuccess<'x> {
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
}

fn pms_union<'x>(mut pmss: Vec<PatternMatchSuccess<'x>>) -> PatternMatchSuccess<'x> {
    fn extract_first_pms<'x>(
        pmrs: &mut Vec<PatternMatchSuccess<'x>>,
    ) -> (RangeInclusive<usize>, Captures<'x>) {
        assert!(pmrs.len() > 1);
        let first = pmrs.remove(0);
        (first.range, first.captures)
    }

    fn union_with_pmr<'x>(
        range: &mut RangeInclusive<usize>,
        captures: &mut Captures<'x>,
        pms: PatternMatchSuccess<'x>,
    ) {
        assert_eq!(*range.end() + 1, *pms.range.start());
        *range = *range.start()..=*pms.range.end();
        for (k, v) in pms.captures {
            if captures.contains_key(k) {
                panic!("TODO: Nice error, tag {} captured multiple times.", k);
            }
            captures.insert(k, v);
        }
    }

    let (mut range, mut captures) = extract_first_pms(&mut pmss);
    for pms in pmss {
        union_with_pmr(&mut range, &mut captures, pms);
    }
    PatternMatchSuccess { range, captures }
}

pub trait Pattern {
    fn match_before<'x>(stream: &'x [Token<'x>], before_index: usize) -> PatternMatchResult<'x>;
    fn match_at<'x>(stream: &'x [Token<'x>], at_index: usize) -> PatternMatchResult<'x>;
}

pub struct PatCaptureAny<const KEY: &'static str>;

impl<const KEY: &'static str> Pattern for PatCaptureAny<KEY> {
    fn match_at<'x>(stream: &'x [Token<'x>], at_index: usize) -> PatternMatchResult<'x> {
        let mut res = PatternMatchSuccess::at(at_index);
        res.captures.insert(KEY, &stream[at_index]);
        Ok(res)
    }

    fn match_before<'x>(stream: &'x [Token<'x>], before_index: usize) -> PatternMatchResult<'x> {
        if before_index < 1 {
            Err(())
        } else {
            Self::match_at(stream, before_index)
        }
    }
}

pub struct PatPlain<const CONTENT: &'static str>;

impl<const CONTENT: &'static str> Pattern for PatPlain<CONTENT> {
    fn match_at<'x>(stream: &'x [Token<'x>], at_index: usize) -> PatternMatchResult<'x> {
        if at_index >= stream.len() {
            Err(())
        } else if stream[at_index] == Token::Plain(CONTENT) {
            Ok(PatternMatchSuccess::at(at_index))
        } else {
            Err(())
        }
    }

    fn match_before<'x>(stream: &'x [Token<'x>], before_index: usize) -> PatternMatchResult<'x> {
        if before_index < 1 {
            Err(())
        } else {
            Self::match_at(stream, before_index)
        }
    }
}

impl<P0: Pattern> Pattern for (P0,) {
    fn match_at<'x>(stream: &'x [Token<'x>], at_index: usize) -> PatternMatchResult<'x> {
        P0::match_at(stream, at_index)
    }

    fn match_before<'x>(stream: &'x [Token<'x>], before_index: usize) -> PatternMatchResult<'x> {
        P0::match_before(stream, before_index)
    }
}

impl<P0: Pattern, P1: Pattern> Pattern for (P0, P1) {
    fn match_at<'x>(stream: &'x [Token<'x>], at_index: usize) -> PatternMatchResult<'x> {
        let base = P0::match_at(stream, at_index)?;
        let next = P1::match_at(stream, *base.range.end() + 1)?;
        Ok(pms_union(vec![base, next]))
    }

    fn match_before<'x>(stream: &'x [Token<'x>], before_index: usize) -> PatternMatchResult<'x> {
        let base = P0::match_at(stream, at_index)?;
        let previous = P1::match_at(stream, *base.range.end() + 1)?;
        Ok(pms_union(vec![base, next]))
    }
}

pub struct PatBi<Before: Pattern, At: Pattern>(PhantomData<(Before, At)>);
