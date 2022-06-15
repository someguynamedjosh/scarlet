use crate::{
    parser::{phrase::Phrase, Node, ParseContext},
    phrase,
};

fn vomit(pc: &ParseContext, src: &Node) -> String {
    format!(
        "{} IS {}",
        src.children[0].as_node().vomit(pc),
        src.children[2].as_node().vomit(pc)
    )
}

pub fn phrase() -> Phrase {
    phrase!(
        "is",
        128, 128,
        None,
        vomit,
        250 => 250, r"\bIS\b", 250
    )
}
