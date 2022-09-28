use crate::{
    definitions::variable::DVariable,
    item::ItemPtr,
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
    Ok(ItemPtr::from_definition(DVariable::new(r#type)))
}

pub fn phrase() -> Phrase {
    phrase!(
        "any",
        128,
        Some((create,)),
        4 => "ANY", 4
    )
}
