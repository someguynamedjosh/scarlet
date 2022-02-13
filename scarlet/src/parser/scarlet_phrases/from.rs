use typed_arena::Arena;

use crate::{
    constructs::{decision::CDecision, ConstructId},
    environment::Environment,
    parser::{
        phrase::{Phrase, UncreateResult},
        Node, NodeChild, ParseContext,
    },
    phrase,
    resolvable::from::RFrom,
    scope::{SPlain, Scope},
    shared::TripleBool,
};

fn create<'x>(
    pc: &ParseContext,
    env: &mut Environment<'x>,
    scope: Box<dyn Scope>,
    node: &Node<'x>,
) -> ConstructId {
    assert_eq!(node.children.len(), 3);
    assert_eq!(node.children[1], NodeChild::Text("FROM"));
    let this = env.push_placeholder(scope);

    let left = node.children[0].as_construct(pc, env, SPlain(this));
    let right = node.children[2].as_construct(pc, env, SPlain(this));
    env.define_unresolved(this, RFrom { left, right });
    this
}

fn uncreate<'a>(
    pc: &ParseContext,
    env: &mut Environment,
    code_arena: &'a Arena<String>,
    uncreate: ConstructId,
    from: &dyn Scope,
) -> UncreateResult<'a> {
    Ok(None)
}

fn vomit(pc: &ParseContext, src: &Node) -> String {
    format!(
        "{} FROM {}",
        src.children[0].as_node().vomit(pc),
        src.children[2].as_node().vomit(pc)
    )
}

pub fn phrase() -> Phrase {
    phrase!(
        "from",
        128, 128,
        Some((create, uncreate)),
        vomit,
        100 => 99, r"FROM", 99
    )
}
