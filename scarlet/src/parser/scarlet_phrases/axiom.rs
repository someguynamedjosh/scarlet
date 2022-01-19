use typed_arena::Arena;

use crate::{
    constructs::{axiom::CAxiom, ConstructId},
    environment::Environment,
    parser::{phrase::Phrase, Node, NodeChild, ParseContext},
    phrase,
    scope::Scope,
};

fn create<'x>(
    pc: &ParseContext,
    env: &mut Environment<'x>,
    scope: Box<dyn Scope>,
    node: &Node<'x>,
) -> ConstructId {
    assert_eq!(node.children.len(), 4);
    assert_eq!(node.children[0], NodeChild::Text("AXIOM"));
    assert_eq!(node.children[1], NodeChild::Text("["));
    assert_eq!(node.children[3], NodeChild::Text("]"));
    let name = node.children[2].as_node().as_ident();
    let con = CAxiom::from_name(env, name);
    env.push_construct(con, scope)
}

fn uncreate<'a>(
    _pc: &ParseContext,
    _env: &mut Environment,
    _code_arena: &'a Arena<String>,
    _uncreate: ConstructId,
    _from: &dyn Scope,
) -> Option<Node<'a>> {
    None
}

fn vomit(_pc: &ParseContext, src: &Node) -> String {
    format!("{:#?}", src)
}

pub fn phrase() -> Phrase {
    phrase!(
        "axiom",
        Some((create, uncreate)),
        vomit,
        4 => r"\bAXIOM\b", r"\[", 255, r"\]"
    )
}
