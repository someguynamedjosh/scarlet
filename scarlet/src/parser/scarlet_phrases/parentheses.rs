use typed_arena::Arena;

use crate::{
    constructs::ItemId,
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
) -> ItemId {
    assert_eq!(node.children.len(), 3);
    assert_eq!(node.children[0], NodeChild::Text("("));
    assert_eq!(node.children[2], NodeChild::Text(")"));
    node.children[1].as_construct_dyn_scope(pc, env, scope)
}

fn uncreate<'a>(
    pc: &ParseContext,
    env: &mut Environment,
    code_arena: &'a Arena<String>,
    uncreate: ItemId,
    from: &dyn Scope,
) -> UncreateResult<'a> {
    Ok(Some(Node {
        phrase: "parentheses",
        children: vec![
            NodeChild::Text("("),
            NodeChild::Node(env.vomit(255, pc, code_arena, uncreate, from)?),
            NodeChild::Text(")"),
        ],
    }))
}

fn vomit(pc: &ParseContext, src: &Node) -> String {
    format!("({})", src.children[1].as_node().vomit(pc))
}

pub fn phrase() -> Phrase {
    phrase!(
        "parentheses",
        128, 255,
        Some((create, uncreate)),
        vomit,
        0 => r"\(", 255, r"\)"
    )
}
