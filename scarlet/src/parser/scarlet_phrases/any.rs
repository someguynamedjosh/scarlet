use crate::{
    definitions::parameter::DParameter,
    item::{IntoItemPtr, ItemPtr},
    parser::{
        phrase::{CreateContext, CreateResult, Phrase},
        Node,
    },
    phrase,
    scope::Scope,
};

pub fn create(ctx: &mut CreateContext, scope: Box<dyn Scope>, node: &Node) -> CreateResult {
    assert_eq!(node.children.len(), 2);
    let r#type = node.children[1].as_item_dyn_scope(ctx, scope.dyn_clone())?;
    Ok(DParameter::new(128, node.position, r#type).into_ptr())
}

pub fn phrase() -> Phrase {
    phrase!(
        "any",
        128,
        Some((create,)),
        4 => "ANY", 4
    )
}
