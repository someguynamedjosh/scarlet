use crate::{
    definitions::identifier::DIdentifier,
    item::{IntoItemPtr, ItemPtr},
    parser::{
        phrase::{CreateContext, CreateResult, Phrase},
        Node,
    },
    phrase,
    scope::Scope,
};

pub fn create(_ctx: &mut CreateContext, _scope: Box<dyn Scope>, node: &Node) -> CreateResult {
    assert_eq!(node.children.len(), 1);
    Ok(DIdentifier::new(node.children[0].as_text().to_owned()).into_ptr())
}

pub fn phrase() -> Phrase {
    phrase!(
        "identifier",
        255,
        Some((create,)),
        4 => r"[a-zA-Z0-9_]+"
    )
}
