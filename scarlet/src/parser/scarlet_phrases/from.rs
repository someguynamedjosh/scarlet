use crate::{
    environment::{vomit::VomitContext, Environment},
    item::{
        resolvable::{from::RFrom, DResolvable},
        Item, ItemDefinition, ItemPtr,
    },
    parser::{
        phrase::{Phrase, UncreateResult},
        Node, NodeChild, ParseContext,
    },
    phrase,
    scope::{SPlain, Scope},
};

fn create(pc: &ParseContext, env: &mut Environment, scope: Box<dyn Scope>, node: &Node) -> ItemPtr {
    assert_eq!(node.children.len(), 3);
    assert_eq!(node.children[1], NodeChild::Text("FROM"));
    let this = Item::placeholder_with_scope(scope);

    let left = node.children[0].as_construct(pc, env, SPlain(this.ptr_clone()));
    let right = node.children[2].as_construct(pc, env, SPlain(this.ptr_clone()));
    this.redefine(DResolvable::new(RFrom { left, right }).clone_into_box());
    this
}

fn uncreate<'a>(
    env: &mut Environment,
    ctx: &mut VomitContext<'a, '_>,
    uncreate: ItemPtr,
) -> UncreateResult<'a> {
    Ok(None)
}

fn vomit(pc: &ParseContext, src: &Node) -> String {
    format!(
        "{} FROM {}",
        src.children[0].as_node().vomit(pc),
        src.children[2].as_node().vomit(pc)
    )
}

pub fn phrase() -> Phrase {
    phrase!(
        "from",
        128, 128,
        Some((create, uncreate)),
        vomit,
        100 => 99, r"FROM", 99
    )
}
