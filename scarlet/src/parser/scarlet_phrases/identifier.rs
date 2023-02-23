use crate::{
    parser::{
        phrase::{CreateContext, CreateResult, Phrase},
        Node,
    },
    phrase,
};

pub fn create(_ctx: &mut CreateContext, node: &Node) -> CreateResult {
    assert_eq!(node.children.len(), 1);
    todo!()
}

pub fn phrase() -> Phrase {
    phrase!(
        "identifier",
        255,
        Some((create,)),
        4 => r"[a-zA-Z0-9_]+"
    )
}
