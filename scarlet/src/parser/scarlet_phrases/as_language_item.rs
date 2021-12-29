use typed_arena::Arena;

use crate::{
    constructs::{unique::CUnique, ConstructId},
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
    pc: &ParseContext,
    env: &mut Environment,
    code_arena: &'a Arena<String>,
    uncreate: ConstructId,
    from: ConstructId,
) -> Option<Node<'a>> {
    None
}

pub fn phrase() -> Phrase {
    phrase!(
        "as language item",
        Some((create, uncreate)),
        4 => 4, r"\.AS_LANGUAGE_ITEM", r"\[", 255, r"\]"
    )
}
