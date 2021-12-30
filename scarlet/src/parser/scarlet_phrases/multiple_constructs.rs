use typed_arena::Arena;

use crate::{
    constructs::{unique::CUnique, ConstructId},
    environment::Environment,
    parser::{phrase::Phrase, Node, NodeChild, ParseContext},
    phrase,
    scope::Scope,
};

fn vomit(pc: &ParseContext, src: &Node) -> String {
    format!(
        "{}  {}",
        src.children[0].as_node().vomit(pc),
        src.children[2].as_node().vomit(pc)
    )
}

pub fn phrase() -> Phrase {
    phrase!(
        "multiple constructs",
        None,
        vomit,
        255 => 255, r",", 255
    )
}
