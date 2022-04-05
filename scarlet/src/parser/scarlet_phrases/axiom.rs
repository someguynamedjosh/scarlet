use typed_arena::Arena;

use crate::{
    constructs::{axiom::CAxiom, ItemId},
    environment::{Environment, vomit::VomitContext},
    parser::{
        phrase::{Phrase, UncreateResult},
        Node, NodeChild, ParseContext,
    },
    phrase,
    scope::Scope,
};

fn create<'x>(
    _pc: &ParseContext,
    env: &mut Environment<'x>,
    scope: Box<dyn Scope>,
    node: &Node<'x>,
) -> ItemId {
    assert_eq!(node.children.len(), 4);
    assert_eq!(node.children[0], NodeChild::Text("AXIOM"));
    assert_eq!(node.children[1], NodeChild::Text("["));
    assert_eq!(node.children[3], NodeChild::Text("]"));
    let name = node.children[2].as_node().as_ident();
    let con = CAxiom::from_name(env, name);
    env.push_construct(con, scope)
}

fn uncreate<'a>(
    env: &mut Environment,
    ctx: &mut VomitContext<'a, '_>,
    uncreate: ItemId,
) -> UncreateResult<'a> {
    if let Some(cax) = env.get_and_downcast_construct_definition::<CAxiom>(uncreate)? {
        let cax = cax.clone();
        let statement = "unknown_statement";
        Ok(Some(Node {
            phrase: "axiom",
            children: vec![
                NodeChild::Text("AXIOM"),
                NodeChild::Text("["),
                NodeChild::Text(statement),
                NodeChild::Text("]"),
            ],
            ..Default::default()
        }))
    } else {
        Ok(None)
    }
}

fn vomit(_pc: &ParseContext, src: &Node) -> String {
    format!("AXIOM[ {} ]", src.children[2].as_text())
}

pub fn phrase() -> Phrase {
    phrase!(
        "axiom",
        128, 128,
        Some((create, uncreate)),
        vomit,
        4 => r"\bAXIOM\b", r"\[", 255, r"\]"
    )
}
