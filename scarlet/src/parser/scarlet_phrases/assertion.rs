use typed_arena::Arena;

use crate::{
    item::{axiom::CAxiom, ItemPtr, assertion::CAssertion},
    environment::{vomit::VomitContext, Environment},
    parser::{
        phrase::{Phrase, UncreateResult},
        Node, NodeChild, ParseContext,
    },
    phrase,
    scope::{Scope, SPlain},
};

fn create(
    pc: &ParseContext,
    env: &mut Environment,
    scope: Box<dyn Scope>,
    node: &Node,
) -> ItemPtr {
    assert_eq!(node.children.len(), 4);
    assert_eq!(node.children[0], NodeChild::Text("ASSERT"));
    assert_eq!(node.children[1], NodeChild::Text("["));
    assert_eq!(node.children[3], NodeChild::Text("]"));
    let this = env.push_placeholder(scope);
    let name = node.children[2].as_construct(pc, env, SPlain(this));
    let con = CAssertion::new(name);
    env.define_item(this, con);
    this
}

fn uncreate<'a>(
    env: &mut Environment,
    ctx: &mut VomitContext<'a, '_>,
    uncreate: ItemPtr,
) -> UncreateResult<'a> {
    if let Some(cax) = env.get_and_downcast_construct_definition::<CAxiom>(uncreate)? {
        let cax = cax.clone();
        let statement = cax.get_statement(env);
        let statement = &statement[..statement.len() - "_statement".len()];
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
    format!("ASSERT[{}]", src.children[2].as_text())
}

pub fn phrase() -> Phrase {
    phrase!(
        "assertion",
        128, 128,
        Some((create, uncreate)),
        vomit,
        4 => r"\bASSERT\b", r"\[", 255, r"\]"
    )
}
