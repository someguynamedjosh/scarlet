use crate::{parser::phrase::Phrase, phrase};

pub fn phrase() -> Phrase {
    phrase!(
        "structure",
        128,
        None,
        4 => r"\[", 255, r"\]"
    )
}
