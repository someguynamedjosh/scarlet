use typed_arena::Arena;

use crate::{
    constructs::{unique::CUnique, ConstructId},
    environment::Environment,
    parser::{phrase::Phrase, Node, NodeChild, ParseContext},
    scope::Scope,
    phrase,
};

fn create<'x>(
    _pc: &ParseContext,
    env: &mut Environment<'x>,
    scope: Box<dyn Scope>,
    node: &Node<'x>,
) -> ConstructId {
    assert_eq!(node.children, &[NodeChild::Text("UNIQUE")]);
    let id = env.push_unique();
    env.push_construct(CUnique::new(id), scope)
}

fn uncreate<'a>(
    pc: &ParseContext,
    env: &mut Environment,
    code_arena: &'a Arena<String>,
    uncreate: ConstructId,
    from: ConstructId,
) -> Option<Node<'a>> {
    todo!()
}

pub fn phrase() -> Phrase {
    phrase!(
        "keyword UNIQUE",
        Some((create, uncreate)),
        0 => r"\bUNIQUE\b"
    )
}
