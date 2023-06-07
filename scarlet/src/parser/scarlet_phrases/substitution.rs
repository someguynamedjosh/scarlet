use crate::{
    definitions::substitution::{DUnresolvedSubstitution, UnresolvedTarget},
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
            subs.push((UnresolvedTarget::Positional, child.as_item(ctx)?));
        }
    }
    let definition = DUnresolvedSubstitution::new(base, subs);
    Ok(ctx.env.define0(definition))
}

pub fn phrase() -> Phrase {
    phrase!(
        "substitution",
        128,
        Some((create,)),
        4 => 4, r"\(", 255, r"\)"
    )
}
