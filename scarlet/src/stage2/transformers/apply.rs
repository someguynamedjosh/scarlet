use std::collections::HashMap;

use super::{basics::ApplyContext, build};
use crate::stage2::transformers::basics::{OwnedOrBorrowed, Precedence, Transformer};

fn apply_transformers_ltr<'t>(c: &mut ApplyContext<'_, 't>, transformers: &[&dyn Transformer]) {
    let mut index = 0;
    while index < c.to.len() {
        for transformer in transformers {
            if transformer.should_be_applied_at(c, index) {
                let result = transformer.apply(c, index);
                if !result.replace_range.contains(&index) {
                    panic!(
                        "Transformer wants to replace {:?}, \
                    which does not contain the original index {}.",
                        result.replace_range, index
                    );
                }
                index = *result.replace_range.start();
                c.to.splice(result.replace_range, std::iter::once(result.with));
            }
        }
        index += 1;
    }
}

fn apply_transformers_rtl<'t>(c: &mut ApplyContext<'_, 't>, transformers: &[&dyn Transformer]) {
    let mut index = c.to.len();
    while index > 0 {
        index -= 1;
        for transformer in transformers {
            if transformer.should_be_applied_at(c, index) {
                let result = transformer.apply(c, index);
                if !result.replace_range.contains(&index) {
                    panic!(
                        "Transformer wants to replace {:?}, \
                    which does not contain the original index {}.",
                        result.replace_range, index
                    );
                }
                index = *result.replace_range.start();
                c.to.splice(result.replace_range, std::iter::once(result.with));
            }
        }
    }
}

pub fn apply_transformers<'e, 't>(
    c: &mut ApplyContext<'_, 't>,
    extras: &'e HashMap<Precedence, Vec<Box<dyn Transformer + 'e>>>,
) {
    for precedence in 0..=u8::MAX {
        let transformers = build::build_transformers(precedence, extras);
        let transformers: Vec<_> = transformers.iter().map(OwnedOrBorrowed::as_ref).collect();
        if precedence % 2 == 0 {
            apply_transformers_ltr(c, &transformers);
        } else {
            apply_transformers_rtl(c, &transformers);
        }
    }
}
