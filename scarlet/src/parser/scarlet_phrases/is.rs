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
        "is",
        None,
        250 => 250, r"IS", 250
    )
}
