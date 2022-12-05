use crate::{
    definitions::parameter::DParameter,
    parser::{
        phrase::{CreateContext, CreateResult, Phrase},
        Node,
    },
    phrase,
};

pub fn create(ctx: &mut CreateContext, node: &Node) -> CreateResult {
    assert_eq!(node.children.len(), 2);
    let r#type = node.children[1].as_item(ctx)?;
    Ok(DParameter::new(128, node.position, r#type.into_lazy()).into_ptr())
}

pub fn phrase() -> Phrase {
    phrase!(
        "any",
        128,
        Some((create,)),
        4 => "ANY", 4
    )
}
