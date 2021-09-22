pub use crate::stage1::ingest::helpers::*;
use crate::stage1::ingest::nom_prelude::*;

/// Returns true if an optional 'public' keyword was specified
pub fn public_parser<'i>() -> impl Parser<'i, bool> {
    let tag = opt(tag_then_ws("public"));
    map(tag, |value| value.is_some())
}
