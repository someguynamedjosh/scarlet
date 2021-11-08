use std::collections::HashMap;

use super::build;
use crate::{
    stage1::transformers::basics::{OwnedOrBorrowed, Precedence, Transformer},
    stage2::structure::Token,
};

fn apply_transformers_ltr<'t>(to: &mut Vec<Token<'t>>, transformers: &[&dyn Transformer]) {
    let mut index = 0;
    while index < to.len() {
        for transformer in transformers {
            if transformer.should_be_applied_at(&to, index) {
                let result = transformer.apply(to, index);
                if !result.replace_range.contains(&index) {
                    panic!(
                        "Transformer wants to replace {:?}, \
                    which does not contain the original index {}.",
                        result.replace_range, index
                    );
                }
                index = *result.replace_range.start();
                to.splice(result.replace_range, std::iter::once(result.with));
            }
        }
        index += 1;
    }
}

fn apply_transformers_rtl<'t>(to: &mut Vec<Token<'t>>, transformers: &[&dyn Transformer]) {
    let mut index = to.len();
    while index > 0 {
        index -= 1;
        for transformer in transformers {
            if transformer.should_be_applied_at(&to, index) {
                let result = transformer.apply(to, index);
                if !result.replace_range.contains(&index) {
                    panic!(
                        "Transformer wants to replace {:?}, \
                    which does not contain the original index {}.",
                        result.replace_range, index
                    );
                }
                index = *result.replace_range.start();
                to.splice(result.replace_range, std::iter::once(result.with));
            }
        }
    }
}

pub fn apply_transformers<'e, 't>(
    to: &mut Vec<Token<'t>>,
    extras: &'e HashMap<Precedence, Vec<Box<dyn Transformer + 'e>>>,
) {
    for precedence in 0..=u8::MAX {
        let transformers = build::build_transformers(precedence, extras);
        let transformers: Vec<_> = transformers.iter().map(OwnedOrBorrowed::as_ref).collect();
        if precedence % 2 == 0 {
            apply_transformers_ltr(to, &transformers);
        } else {
            apply_transformers_rtl(to, &transformers);
        }
    }
}
