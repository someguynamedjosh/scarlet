use crate::{parser::phrase::Phrase, phrase};

pub fn phrase() -> Phrase {
    phrase!(
        "any",
        128,
        None,
        4 => "ANY", 4
    )
}
