use typed_arena::Arena;

use crate::{
    constructs::{unique::CUnique, ConstructId, equal::CEqual},
    environment::Environment,
    parser::{phrase::Phrase, Node, NodeChild, ParseContext},
    phrase,
    scope::{Scope, SPlain},
};

fn create<'x>(
    pc: &ParseContext,
    env: &mut Environment<'x>,
    scope: Box<dyn Scope>,
    node: &Node<'x>,
) -> ConstructId {
    assert_eq!(node.children.len(), 3);
    assert_eq!(node.children[1], NodeChild::Text("="));
    let this = env.push_placeholder(scope);
    let left = node.children[0].as_construct(pc, env, SPlain(this));
    let right = node.children[2].as_construct(pc, env, SPlain(this));
    env.define_construct(this, CEqual::new(left, right));
    this
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
        "equal operator",
        Some((create, uncreate)),
        65 => 65, r"=", 65
    )
}
