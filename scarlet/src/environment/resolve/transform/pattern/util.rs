use std::ops::RangeInclusive;

use super::PatternMatchSuccess;
use crate::environment::resolve::transform::pattern::Captures;

pub(super) fn pms_union<'i, 'x>(
    mut pmss: Vec<PatternMatchSuccess<'i, 'x>>,
) -> PatternMatchSuccess<'i, 'x> {
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
