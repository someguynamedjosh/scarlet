use typed_arena::Arena;

use crate::{
    constructs::{unique::CUnique, ConstructId},
    environment::Environment,
    parser::{phrase::Phrase, Node, NodeChild, ParseContext},
    scope::Scope,
    phrase,
};

pub fn phrase() -> Phrase {
    phrase!(
        "multiple constructs",
        None,
        255 => 255, r",", 255
    )
}
