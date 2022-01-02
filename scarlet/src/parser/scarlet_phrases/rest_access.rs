use typed_arena::Arena;

use crate::{
    constructs::{unique::CUnique, ConstructId, structt::{CAtomicStructMember, AtomicStructMember}},
    environment::Environment,
    parser::{phrase::{Phrase}, Node, NodeChild, ParseContext},
    scope::{Scope, SPlain},
    phrase,
};

fn create<'x>(
    pc: &ParseContext,
    env: &mut Environment<'x>,
    scope: Box<dyn Scope>,
    node: &Node<'x>,
) -> ConstructId {
    assert_eq!(node.children.len(), 2);
    let this = env.push_placeholder(scope);
    let base = node.children[0].as_construct(pc, env, SPlain(this));
    env.define_construct(this, CAtomicStructMember(base, AtomicStructMember::Rest));
    this
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

fn vomit(pc: &ParseContext, src: &Node) -> String {
    format!("{:#?}", src)
}

pub fn phrase() -> Phrase {
    phrase!(
        "rest access",
        Some((create, uncreate)),
        vomit,
        4 => 4, r"\.REST"
    )
}
