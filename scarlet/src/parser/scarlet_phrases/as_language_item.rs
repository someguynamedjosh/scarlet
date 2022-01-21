use typed_arena::Arena;

use crate::{
    constructs::ConstructId,
    environment::Environment,
    parser::{phrase::Phrase, Node, NodeChild, ParseContext},
    phrase,
    scope::Scope,
};

fn create<'x>(
    pc: &ParseContext,
    env: &mut Environment<'x>,
    scope: Box<dyn Scope>,
    node: &Node<'x>,
) -> ConstructId {
    assert_eq!(node.children.len(), 5);
    assert_eq!(node.children[1], NodeChild::Text(".AS_LANGUAGE_ITEM"));
    assert_eq!(node.children[2], NodeChild::Text("["));
    assert_eq!(node.children[4], NodeChild::Text("]"));
    let base = node.children[0].as_construct_dyn_scope(pc, env, scope);
    let name = node.children[3].as_node().as_ident();
    env.define_language_item(name, base);
    base
}

fn uncreate<'a>(
    _pc: &ParseContext,
    _env: &mut Environment,
    _code_arena: &'a Arena<String>,
    _uncreate: ConstructId,
    _from: &dyn Scope,
) -> Option<Node<'a>> {
    None
}

fn vomit(_pc: &ParseContext, src: &Node) -> String {
    format!("{:#?}", src)
}

pub fn phrase() -> Phrase {
    phrase!(
        "as language item",
        128, 128,
        Some((create, uncreate)),
        vomit,
        4 => 4, r"\.AS_LANGUAGE_ITEM", r"\[", 255, r"\]"
    )
}
