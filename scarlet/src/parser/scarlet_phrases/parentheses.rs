use typed_arena::Arena;

use crate::{
    constructs::{unique::CUnique, ConstructId},
    environment::Environment,
    parser::{phrase::Phrase, Node, NodeChild, ParseContext},
    scope::Scope,
    phrase,
};

fn create<'x>(
    pc: &ParseContext,
    env: &mut Environment<'x>,
    scope: Box<dyn Scope>,
    node: &Node<'x>,
) -> ConstructId {
    assert_eq!(node.children.len(), 3);
    assert_eq!(node.children[0], NodeChild::Text("("));
    assert_eq!(node.children[2], NodeChild::Text(")"));
    node.children[1].as_construct_dyn_scope(pc, env, scope)
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
        "parentheses",
        Some((create, uncreate)),
        0 => r"\(", 255, r"\)"
    )
}
