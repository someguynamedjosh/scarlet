use crate::{
    parser::{phrase::Phrase, Node, ParseContext},
    phrase,
};

fn vomit(pc: &ParseContext, src: &Node) -> String {
    format!("PROOF({})", src.children[2].vomit(pc))
}

pub fn phrase() -> Phrase {
    phrase!(
        "proof_target",
        128, 128,
        None,
        vomit,
        0 => r"\b(PROOF|PR)\b" , r"\(", 255, r"\)"
    )
}
