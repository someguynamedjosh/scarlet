use typed_arena::Arena;

use crate::{
    constructs::{unique::CUnique, ConstructId},
    environment::Environment,
    parser::{phrase::{Phrase}, Node, NodeChild, ParseContext},
    scope::Scope,
    phrase,
};

fn vomit(pc: &ParseContext, src: &Node) -> String {
    format!("{:#?}", src)
}

pub fn phrase() -> Phrase {
    phrase!(
        "is",
        None,
        vomit,
        250 => 250, r"IS", 250
    )
}
