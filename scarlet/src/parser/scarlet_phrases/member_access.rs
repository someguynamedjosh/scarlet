use crate::{
    definitions::member_access::DMemberAccess,
    item::IntoRef,
    parser::{
        phrase::{CreateContext, CreateResult, Phrase},
        Node,
    },
    phrase,
};

pub fn create(ctx: &mut CreateContext, node: &Node) -> CreateResult {
    assert_eq!(node.children.len(), 3);
    let base = node.children[0].as_item(ctx)?;
    let member_name = node.children[2].as_ident()?;
    Ok(DMemberAccess::new(base, member_name.to_owned()).into_ref(node.position))
}

pub fn phrase() -> Phrase {
    phrase!(
        "member access",
        128,
        Some((create,)),
        4 => 4, r"\.", 4
    )
}
