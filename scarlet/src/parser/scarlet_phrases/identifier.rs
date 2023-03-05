use crate::{
    definitions::identifier::DIdentifier,
    parser::{
        phrase::{CreateContext, CreateResult, Phrase},
        Node,
    },
    phrase,
};

pub fn create(ctx: &mut CreateContext, node: &Node) -> CreateResult {
    assert_eq!(node.children.len(), 1);
    let definition = DIdentifier::new(node.children[0].as_text().to_owned());
    Ok(ctx.env.new_defined_item(definition))
}

pub fn phrase() -> Phrase {
    phrase!(
        "identifier",
        255,
        Some((create,)),
        4 => r"[a-zA-Z0-9_]+"
    )
}
