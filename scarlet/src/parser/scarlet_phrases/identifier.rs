use crate::{parser::phrase::Phrase, phrase};

pub fn phrase() -> Phrase {
    phrase!(
        "identifier",
        255,
        None,
        4 => r"[a-zA-Z0-9]+"
    )
}
