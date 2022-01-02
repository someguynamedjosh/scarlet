use typed_arena::Arena;

use crate::{
    constructs::{unique::CUnique, ConstructId},
    environment::Environment,
    parser::{phrase::Phrase, Node, NodeChild, ParseContext},
    phrase,
    resolvable::RIdentifier,
    scope::Scope,
};

fn create<'x>(
    _pc: &ParseContext,
    env: &mut Environment<'x>,
    scope: Box<dyn Scope>,
    node: &Node<'x>,
) -> ConstructId {
    assert_eq!(node.phrase, "identifier");
    assert_eq!(node.children.len(), 1);
    env.push_unresolved(RIdentifier(node.children[0].as_text()), scope)
}

fn uncreate<'a>(
    pc: &ParseContext,
    env: &mut Environment,
    code_arena: &'a Arena<String>,
    uncreate: ConstructId,
    from: &dyn Scope,
) -> Option<Node<'a>> {
    None
}

fn vomit(_pc: &ParseContext, src: &Node) -> String {
    format!("{}", src.children[0].as_text())
}

pub fn phrase() -> Phrase {
    phrase!(
        "identifier",
        Some((create, uncreate)),
        vomit,
        0 => r"[a-zA-Z0-9_]+"
    )
}
