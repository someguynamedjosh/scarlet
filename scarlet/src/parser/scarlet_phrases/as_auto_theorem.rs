use crate::{
    environment::{vomit::VomitContext, Environment},
    item::ItemPtr,
    parser::{
        phrase::{Phrase, UncreateResult},
        Node, NodeChild, ParseContext,
    },
    phrase,
    scope::Scope,
};

fn create(pc: &ParseContext, env: &mut Environment, scope: Box<dyn Scope>, node: &Node) -> ItemPtr {
    assert_eq!(node.children.len(), 2);
    assert_eq!(node.children[1], NodeChild::Text(".AS_AUTO_THEOREM"));
    let base = node.children[0].as_construct_dyn_scope(pc, env, scope);
    env.add_auto_theorem(base.ptr_clone());
    base
}

fn uncreate<'a>(
    _env: &mut Environment,
    _ctx: &mut VomitContext<'a, '_>,
    _uncreate: ItemPtr,
) -> UncreateResult<'a> {
    Ok(None)
}

fn vomit(_pc: &ParseContext, src: &Node) -> String {
    format!("{:#?}", src)
}

pub fn phrase() -> Phrase {
    phrase!(
        "as auto theorem",
        128, 128,
        Some((create, uncreate)),
        vomit,
        4 => 4, r"\.AS_AUTO_THEOREM"
    )
}
