use crate::{
    definitions::struct_literal::DStructLiteral,
    item::IntoItemPtr,
    parser::{
        phrase::{CreateContext, CreateResult, Phrase},
        util::collect_comma_list,
        Node,
    },
    phrase,
};

pub fn create(ctx: &mut CreateContext, node: &Node) -> CreateResult {
    assert_eq!(node.children.len(), 3);
    let mut fields = Vec::new();
    for child in collect_comma_list(&node.children[1]) {
        if let Some(is) = child.as_is() {
            let (label, value) = is?;
            fields.push((label.to_owned(), value.as_item(ctx)?.into_lazy()));
        } else {
            fields.push((String::new(), child.as_item(ctx)?.into_lazy()));
        }
    }
    Ok(DStructLiteral::new_module(fields).into_ptr())
}

pub fn phrase() -> Phrase {
    phrase!(
        "structure",
        128,
        Some((create,)),
        4 => r"\[", 255, r"\]"
    )
}
