use typed_arena::Arena;

use crate::{
    constructs::{is_populated_struct::CIsPopulatedStruct, ItemId},
    environment::{vomit::VomitContext, Environment},
    parser::{
        phrase::{Phrase, UncreateResult},
        Node, NodeChild, ParseContext,
    },
    phrase,
    scope::{SPlain, Scope},
};

fn create<'x>(
    pc: &ParseContext,
    env: &mut Environment<'x>,
    scope: Box<dyn Scope>,
    node: &Node<'x>,
) -> ItemId {
    assert_eq!(node.children.len(), 2);
    assert_eq!(node.children[1], NodeChild::Text(".IS_POPULATED_STRUCT"));
    let this = env.push_placeholder(scope);
    let base = node.children[0].as_construct(pc, env, SPlain(this));
    env.define_item(this, CIsPopulatedStruct::new(base));
    this
}

fn uncreate<'a>(
    env: &mut Environment,
    ctx: &mut VomitContext<'a, '_>,
    uncreate: ItemId,
) -> UncreateResult<'a> {
    Ok(
        if let Some(cips) =
            env.get_and_downcast_construct_definition::<CIsPopulatedStruct>(uncreate)?
        {
            let cips = cips.clone();
            Some(Node {
                phrase: "is populated struct",
                children: vec![NodeChild::Node(env.vomit(4, ctx, cips.get_base()))],
                ..Default::default()
            })
        } else {
            None
        },
    )
}

fn vomit(pc: &ParseContext, src: &Node) -> String {
    format!("{}.IS_POPULATED_STRUCT", src.children[0].vomit(pc))
}

pub fn phrase() -> Phrase {
    phrase!(
        "is populated struct",
        128, 128,
        Some((create, uncreate)),
        vomit,
        4 => 4, r"\.IS_POPULATED_STRUCT"
    )
}
