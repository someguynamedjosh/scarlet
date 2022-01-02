use typed_arena::Arena;

use crate::{
    constructs::{
        structt::{AtomicStructMember, CAtomicStructMember},
        unique::CUnique,
        ConstructId,
    },
    environment::Environment,
    parser::{phrase::{Phrase}, Node, NodeChild, ParseContext},
    phrase,
    scope::{SPlain, Scope},
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
    env.define_construct(this, CAtomicStructMember(base, AtomicStructMember::Value));
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
        "value access",
        Some((create, uncreate)),
        vomit,
        4 => 4, r"\.VALUE"
    )
}
