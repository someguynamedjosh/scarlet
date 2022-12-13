use crate::{parser::phrase::Phrase, phrase};

pub fn phrase() -> Phrase {
    phrase!(
        "multiple items",
        128,
        None,
        252 => 252, r",", 252
    )
}
