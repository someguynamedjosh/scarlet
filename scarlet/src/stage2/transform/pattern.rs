use std::{collections::HashMap, marker::PhantomData, ops::RangeInclusive, slice::SliceIndex};

use crate::stage2::structure::Token;

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

fn pms_union<'i, 'x>(mut pmss: Vec<PatternMatchSuccess<'i, 'x>>) -> PatternMatchSuccess<'i, 'x> {
    fn extract_first_pms<'i, 'x>(
        pmrs: &mut Vec<PatternMatchSuccess<'i, 'x>>,
    ) -> (RangeInclusive<usize>, Captures<'i, 'x>) {
        assert!(pmrs.len() > 1);
        let first = pmrs.remove(0);
        (first.range, first.captures)
    }

    fn union_with_pmr<'i, 'x>(
        range: &mut RangeInclusive<usize>,
        captures: &mut Captures<'i, 'x>,
        pms: PatternMatchSuccess<'i, 'x>,
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
    fn match_before<'i, 'x>(
        &self,
        stream: &'i [Token<'x>],
        before_index: usize,
    ) -> PatternMatchResult<'i, 'x>;
    fn match_at<'i, 'x>(
        &self,
        stream: &'i [Token<'x>],
        at_index: usize,
    ) -> PatternMatchResult<'i, 'x>;
}

impl<P: Pattern> Pattern for &P {
    fn match_at<'i, 'x>(
        &self,
        stream: &'i [Token<'x>],
        at_index: usize,
    ) -> PatternMatchResult<'i, 'x> {
        (*self).match_at(stream, at_index)
    }

    fn match_before<'i, 'x>(
        &self,
        stream: &'i [Token<'x>],
        before_index: usize,
    ) -> PatternMatchResult<'i, 'x> {
        (*self).match_before(stream, before_index)
    }
}

pub struct PatCaptureAny {
    pub key: &'static str,
}

impl Pattern for PatCaptureAny {
    fn match_at<'i, 'x>(
        &self,
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
        stream: &'i [Token<'x>],
        before_index: usize,
    ) -> PatternMatchResult<'i, 'x> {
        if before_index < 1 {
            Err(())
        } else {
            Self::match_at(&self, stream, before_index - 1)
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
        stream: &'i [Token<'x>],
        before_index: usize,
    ) -> PatternMatchResult<'i, 'x> {
        if before_index < 1 {
            Err(())
        } else {
            Self::match_at(&self, stream, before_index - 1)
        }
    }
}

pub struct PatPlain(pub &'static str);

impl Pattern for PatPlain {
    fn match_at<'i, 'x>(
        &self,
        stream: &'i [Token<'x>],
        at_index: usize,
    ) -> PatternMatchResult<'i, 'x> {
        if at_index >= stream.len() {
            Err(())
        } else if stream[at_index] == Token::Plain(self.0) {
            Ok(PatternMatchSuccess::at(at_index))
        } else {
            Err(())
        }
    }

    fn match_before<'i, 'x>(
        &self,
        stream: &'i [Token<'x>],
        before_index: usize,
    ) -> PatternMatchResult<'i, 'x> {
        if before_index < 1 {
            Err(())
        } else {
            Self::match_at(&self, stream, before_index - 1)
        }
    }
}

pub struct PatPreceded<Before: Pattern, At: Pattern>(Before, At);

impl<Before: Pattern, At: Pattern> Pattern for PatPreceded<Before, At> {
    fn match_at<'i, 'x>(
        &self,
        stream: &'i [Token<'x>],
        at_index: usize,
    ) -> PatternMatchResult<'i, 'x> {
        Ok(pms_union(vec![
            self.0.match_before(stream, at_index)?,
            self.1.match_at(stream, at_index)?,
        ]))
    }

    fn match_before<'i, 'x>(
        &self,
        stream: &'i [Token<'x>],
        before_index: usize,
    ) -> PatternMatchResult<'i, 'x> {
        Ok(pms_union(vec![
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

impl<P0: Pattern> Pattern for (P0,) {
    fn match_at<'i, 'x>(
        &self,
        stream: &'i [Token<'x>],
        at_index: usize,
    ) -> PatternMatchResult<'i, 'x> {
        self.0.match_at(stream, at_index)
    }

    fn match_before<'i, 'x>(
        &self,
        stream: &'i [Token<'x>],
        before_index: usize,
    ) -> PatternMatchResult<'i, 'x> {
        self.0.match_before(stream, before_index)
    }
}

impl<P0: Pattern, P1: Pattern> Pattern for (P0, P1) {
    fn match_at<'i, 'x>(
        &self,
        stream: &'i [Token<'x>],
        at_index: usize,
    ) -> PatternMatchResult<'i, 'x> {
        let base = self.0.match_at(stream, at_index)?;
        let next = self.1.match_at(stream, *base.range.end() + 1)?;
        Ok(pms_union(vec![base, next]))
    }

    fn match_before<'i, 'x>(
        &self,
        stream: &'i [Token<'x>],
        before_index: usize,
    ) -> PatternMatchResult<'i, 'x> {
        let base = self.0.match_before(stream, before_index)?;
        let previous = self.1.match_before(stream, *base.range.start())?;
        Ok(pms_union(vec![previous, base]))
    }
}

impl<P0: Pattern, P1: Pattern, P2: Pattern> Pattern for (P0, P1, P2) {
    fn match_at<'i, 'x>(
        &self,
        stream: &'i [Token<'x>],
        at_index: usize,
    ) -> PatternMatchResult<'i, 'x> {
        (&self.0, (&self.1, &self.2)).match_at(stream, at_index)
    }

    fn match_before<'i, 'x>(
        &self,
        stream: &'i [Token<'x>],
        before_index: usize,
    ) -> PatternMatchResult<'i, 'x> {
        (&self.0, (&self.1, &self.2)).match_before(stream, before_index)
    }
}

impl<P0: Pattern, P1: Pattern, P2: Pattern, P3: Pattern> Pattern for (P0, P1, P2, P3) {
    fn match_at<'i, 'x>(
        &self,
        stream: &'i [Token<'x>],
        at_index: usize,
    ) -> PatternMatchResult<'i, 'x> {
        (&self.0, (&self.1, &self.2, &self.3)).match_at(stream, at_index)
    }

    fn match_before<'i, 'x>(
        &self,
        stream: &'i [Token<'x>],
        before_index: usize,
    ) -> PatternMatchResult<'i, 'x> {
        (&self.0, (&self.1, &self.2, &self.3)).match_before(stream, before_index)
    }
}

impl<P0: Pattern, P1: Pattern, P2: Pattern, P3: Pattern, P4: Pattern> Pattern
    for (P0, P1, P2, P3, P4)
{
    fn match_at<'i, 'x>(
        &self,
        stream: &'i [Token<'x>],
        at_index: usize,
    ) -> PatternMatchResult<'i, 'x> {
        (&self.0, (&self.1, &self.2, &self.3, &self.4)).match_at(stream, at_index)
    }

    fn match_before<'i, 'x>(
        &self,
        stream: &'i [Token<'x>],
        before_index: usize,
    ) -> PatternMatchResult<'i, 'x> {
        (&self.0, (&self.1, &self.2, &self.3, &self.4)).match_before(stream, before_index)
    }
}

impl<P0: Pattern, P1: Pattern, P2: Pattern, P3: Pattern, P4: Pattern, P5: Pattern> Pattern
    for (P0, P1, P2, P3, P4, P5)
{
    fn match_at<'i, 'x>(
        &self,
        stream: &'i [Token<'x>],
        at_index: usize,
    ) -> PatternMatchResult<'i, 'x> {
        (&self.0, (&self.1, &self.2, &self.3, &self.4, &self.5)).match_at(stream, at_index)
    }

    fn match_before<'i, 'x>(
        &self,
        stream: &'i [Token<'x>],
        before_index: usize,
    ) -> PatternMatchResult<'i, 'x> {
        (&self.0, (&self.1, &self.2, &self.3, &self.4, &self.5)).match_before(stream, before_index)
    }
}

impl<P0: Pattern, P1: Pattern, P2: Pattern, P3: Pattern, P4: Pattern, P5: Pattern, P6: Pattern>
    Pattern for (P0, P1, P2, P3, P4, P5, P6)
{
    fn match_at<'i, 'x>(
        &self,
        stream: &'i [Token<'x>],
        at_index: usize,
    ) -> PatternMatchResult<'i, 'x> {
        (
            &self.0,
            (&self.1, &self.2, &self.3, &self.4, &self.5, &self.6),
        )
            .match_at(stream, at_index)
    }

    fn match_before<'i, 'x>(
        &self,
        stream: &'i [Token<'x>],
        before_index: usize,
    ) -> PatternMatchResult<'i, 'x> {
        (
            &self.0,
            (&self.1, &self.2, &self.3, &self.4, &self.5, &self.6),
        )
            .match_before(stream, before_index)
    }
}
