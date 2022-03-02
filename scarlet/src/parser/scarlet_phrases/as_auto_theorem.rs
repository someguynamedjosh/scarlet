use typed_arena::Arena;

use crate::{
    constructs::ConstructId,
    environment::Environment,
    parser::{
        phrase::{Phrase, UncreateResult},
        Node, NodeChild, ParseContext,
    },
    phrase,
    scope::Scope,
};

fn create<'x>(
    pc: &ParseContext,
    env: &mut Environment<'x>,
    scope: Box<dyn Scope>,
    node: &Node<'x>,
) -> ConstructId {
    assert_eq!(node.children.len(), 2);
    assert_eq!(node.children[1], NodeChild::Text(".AS_AUTO_THEOREM"));
    let base = node.children[0].as_construct_dyn_scope(pc, env, scope);
    env.add_auto_theorem(base);
    base
}

fn uncreate<'a>(
    _pc: &ParseContext,
    _env: &mut Environment,
    _code_arena: &'a Arena<String>,
    _uncreate: ConstructId,
    _from: &dyn Scope,
) -> UncreateResult<'a> {
    Ok(None)
}

fn vomit(_pc: &ParseContext, src: &Node) -> String {
    format!("{:#?}", src)
}

pub fn phrase() -> Phrase {
    phrase!(
        "as auto theorem",
        128, 128,
        Some((create, uncreate)),
        vomit,
        4 => 4, r"\.AS_AUTO_THEOREM"
    )
}
