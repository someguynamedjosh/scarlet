use typed_arena::Arena;

use crate::{
    constructs::{unique::CUnique, ConstructId, is_populated_struct::CIsPopulatedStruct},
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
    assert_eq!(node.children.len(), 2);
    assert_eq!(node.children[1], NodeChild::Text(".IS_POPULATED_STRUCT"));
    let this = env.push_placeholder(scope);
    let base = node.children[0].as_construct(pc, env, SPlain(this));
    env.define_construct(this, CIsPopulatedStruct::new(base));
    this
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
        "is populated struct",
        Some((create, uncreate)),
        4 => 4, r"\.IS_POPULATED_STRUCT"
    )
}
