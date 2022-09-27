use crate::{parser::phrase::Phrase, phrase};

pub fn phrase() -> Phrase {
    phrase!(
        "as language item",
        128,
        None,
        236 => 236, r"AS_LANGUAGE_ITEM", r"\(", 255, r"\)"
    )
}
