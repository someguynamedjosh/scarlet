use crate::{parser::phrase::Phrase, phrase};

pub fn phrase() -> Phrase {
    phrase!(
        "builtin",
        128,
        None,
        4 => "BUILTIN", r"\(", 255, r"\)"
    )
}
