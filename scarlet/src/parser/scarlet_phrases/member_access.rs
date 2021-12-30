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
    todo!()
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

fn vomit(pc: &ParseContext, src: &Node) -> String {
    format!("{:#?}", src)
}

pub fn phrase() -> Phrase {
    phrase!(
        "member access",
        Some((create, uncreate)),
        vomit,
        4 => 4, r"\.", 4
    )
}
