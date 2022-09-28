use crate::{
    parser::{
        phrase::{CreateContext, CreateResult, Phrase},
        Node,
    },
    phrase,
    scope::Scope,
};

pub fn create(ctx: &mut CreateContext, scope: Box<dyn Scope>, node: &Node) -> CreateResult {
    assert_eq!(node.children.len(), 5);
    let definition = node.children[0].as_item_dyn_scope(ctx, scope.dyn_clone())?;
    let name = node.children[3].as_ident()?;
    ctx.env
        .define_language_item(name, definition.ptr_clone())
        .map_err(|err| err.with_source_code_block_error(node.position))?;
    Ok(definition)
}

pub fn phrase() -> Phrase {
    phrase!(
        "as language item",
        128,
        Some((create,)),
        236 => 236, r"AS_LANGUAGE_ITEM", r"\(", 255, r"\)"
    )
}
