use typed_arena::Arena;

use crate::{
    environment::{vomit::VomitContext, Environment},
    item::{definitions::unique::DUnique, ItemPtr},
    parser::{
        phrase::{Phrase, UncreateResult},
        Node, NodeChild, ParseContext,
    },
    phrase,
    scope::Scope,
};

fn create(
    _pc: &ParseContext,
    env: &mut Environment,
    scope: Box<dyn Scope>,
    node: &Node,
) -> ItemPtr {
    assert_eq!(node.children, &[NodeChild::Text("UNIQUE")]);
    let id = env.push_unique();
    env.push_construct(DUnique::new(id), scope)
}

fn uncreate<'a>(
    env: &mut Environment,
    ctx: &mut VomitContext<'a, '_>,
    uncreate: ItemPtr,
) -> UncreateResult<'a> {
    Ok(
        if let Some(..) = env.get_and_downcast_construct_definition::<DUnique>(uncreate)? {
            let node = Node {
                phrase: "UNIQUE",
                children: vec![NodeChild::Text("UNIQUE")],
                ..Default::default()
            };
            let name = ctx.get_name(env, uncreate, || node);
            Some(Node {
                phrase: "identifier",
                children: vec![NodeChild::Text(name)],
                ..Default::default()
            })
        } else {
            None
        },
    )
}

fn vomit(_pc: &ParseContext, src: &Node) -> String {
    src.children[0].as_text().to_owned()
}

pub fn phrase() -> Phrase {
    phrase!(
        "UNIQUE",
        128, 128,
        Some((create, uncreate)),
        vomit,
        0 => r"\bUNIQUE\b"
    )
}
