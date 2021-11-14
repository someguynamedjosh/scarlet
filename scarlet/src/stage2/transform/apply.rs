use std::collections::HashMap;

use super::{basics::ApplyContext, build};
use crate::stage2::{
    structure::Token,
    transform::basics::{OwnedOrBorrowed, Precedence, Transformer},
};

fn apply_transformers_ltr<'t>(
    c: &mut ApplyContext<'_, 't>,
    stream: &mut Vec<Token<'t>>,
    transformers: &[&dyn Transformer],
) {
    let mut index = 0;
    while index < stream.len() {
        for transformer in transformers {
            if let Ok(success) = transformer.pattern().match_at(&stream[..], index) {
                if !success.range.contains(&index) {
                    panic!(
                        "Transformer wants to replace {:?}, \
                    which does not contain the original index {}.",
                        success.range, index
                    );
                }
                index = *success.range.start();

                let result = transformer.apply(c, index);
                stream.splice(success.range, std::iter::once(result.0));
            }
        }
        index += 1;
    }
}

fn apply_transformers_rtl<'t>(
    c: &mut ApplyContext<'_, 't>,
    stream: &mut Vec<Token<'t>>,
    transformers: &[&dyn Transformer],
) {
    let mut index = stream.len();
    while index > 0 {
        index -= 1;
        for transformer in transformers {
            if let Ok(success) = transformer.pattern().match_at(&stream[..], index) {
                if !success.range.contains(&index) {
                    panic!(
                        "Transformer wants to replace {:?}, \
                    which does not contain the original index {}.",
                        success.range, index
                    );
                }
                index = *success.range.start();

                let result = transformer.apply(c, index);
                stream.splice(success.range, std::iter::once(result.0));
            }
        }
    }
}

pub fn apply_transformers<'e, 't>(
    c: &mut ApplyContext<'_, 't>,
    stream: &mut Vec<Token<'t>>,
    extras: &'e HashMap<Precedence, Vec<Box<dyn Transformer + 'e>>>,
) {
    for precedence in 0..=u8::MAX {
        let transformers = build::build_transformers(precedence, extras);
        let transformers: Vec<_> = transformers.iter().map(OwnedOrBorrowed::as_ref).collect();
        if precedence % 2 == 0 {
            apply_transformers_ltr(c, stream, &transformers);
        } else {
            apply_transformers_rtl(c, stream, &transformers);
        }
    }
}
