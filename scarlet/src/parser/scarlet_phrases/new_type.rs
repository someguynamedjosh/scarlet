use crate::{parser::phrase::Phrase, phrase};

pub fn phrase() -> Phrase {
    phrase!(
        "new type",
        128,
        None,
        4 => "NEW_TYPE", r"\(", 255, r"\)"
    )
}
