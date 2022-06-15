use crate::{
    environment::{vomit::VomitContext, Environment},
    item::{definitions::unique::DUnique, Item, ItemDefinition, ItemPtr},
    parser::{
        phrase::{Phrase, UncreateResult},
        Node, NodeChild, ParseContext,
    },
    phrase,
    scope::Scope,
};

fn create(
    _pc: &ParseContext,
    _env: &mut Environment,
    scope: Box<dyn Scope>,
    node: &Node,
) -> ItemPtr {
    assert_eq!(node.children, &[NodeChild::Text("UNIQUE")]);
    Item::new_boxed(DUnique::new().clone_into_box(), scope)
}

fn uncreate<'a>(
    env: &mut Environment,
    ctx: &mut VomitContext<'a, '_>,
    uncreate: ItemPtr,
) -> UncreateResult<'a> {
    Ok(
        if let Some(..) = uncreate.downcast_definition::<DUnique>() {
            let node = Node {
                phrase: "UNIQUE",
                children: vec![NodeChild::Text("UNIQUE")],
                ..Default::default()
            };
            let name = ctx.get_name(env, uncreate.ptr_clone(), || node);
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
