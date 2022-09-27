use crate::{parser::phrase::Phrase, phrase};

pub fn phrase() -> Phrase {
    phrase!(
        "is",
        128,
        None,
        248 => 248, "IS", 248
    )
}
