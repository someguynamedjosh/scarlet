use crate::{
    definitions::identifier::DIdentifier,
    item::ItemPtr,
    parser::{
        phrase::{CreateContext, CreateResult, Phrase},
        Node,
    },
    phrase,
    scope::Scope,
};

pub fn create(_ctx: &mut CreateContext, _scope: Box<dyn Scope>, node: &Node) -> CreateResult {
    assert_eq!(node.children.len(), 1);
    Ok(ItemPtr::from_definition(DIdentifier::new(
        node.children[0].as_text().to_owned(),
    )))
}

pub fn phrase() -> Phrase {
    phrase!(
        "identifier",
        255,
        Some((create,)),
        4 => r"[a-zA-Z0-9_]+"
    )
}
