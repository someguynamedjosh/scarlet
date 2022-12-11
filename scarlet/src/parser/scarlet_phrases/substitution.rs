use crate::{
    definitions::substitution::{DSubstitution, UnresolvedTarget},
    item::IntoItemPtr,
    parser::{
        phrase::{CreateContext, CreateResult, Phrase},
        util::collect_comma_list,
        Node,
    },
    phrase,
    shared::OrderedMap,
};

pub fn create(ctx: &mut CreateContext, node: &Node) -> CreateResult {
    assert_eq!(node.children.len(), 4);
    let base = node.children[0].as_item(ctx)?;
    let mut subs = Vec::new();
    for child in collect_comma_list(&node.children[2]) {
        if let Some(is) = child.as_is() {
            let (label, value) = is?;
            subs.push((
                UnresolvedTarget::Named(label.to_owned()),
                value.as_item(ctx)?,
            ));
        } else {
            subs.push((
                UnresolvedTarget::Positional,
                child.as_item(ctx)?,
            ));
        }
    }
    Ok(DSubstitution::new_unresolved(base, subs).into_ptr())
}

pub fn phrase() -> Phrase {
    phrase!(
        "substitution",
        128,
        Some((create,)),
        4 => 4, r"\(", 255, r"\)"
    )
}
