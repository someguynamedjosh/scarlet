use crate::{
    parser::{phrase::Phrase, Node, ParseContext},
    phrase,
};

fn vomit(_pc: &ParseContext, src: &Node) -> String {
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
