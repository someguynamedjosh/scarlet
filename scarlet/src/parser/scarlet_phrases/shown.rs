use typed_arena::Arena;

use crate::{
    environment::{vomit::VomitContext, Environment},
    item::{definitions::shown::DShown, ItemPtr},
    parser::{
        phrase::{Phrase, UncreateResult},
        Node, NodeChild, ParseContext,
    },
    phrase,
    scope::{SPlain, Scope},
};

fn create(pc: &ParseContext, env: &mut Environment, scope: Box<dyn Scope>, node: &Node) -> ItemPtr {
    assert_eq!(node.children.len(), 2);
    assert_eq!(node.children[1], NodeChild::Text(".SHOWN"));
    let this = env.push_placeholder(scope);
    let base = node.children[0].as_construct(pc, env, SPlain(this));
    env.define_item(this, DShown::new(base));
    this
}

fn uncreate<'a>(
    env: &mut Environment,
    ctx: &mut VomitContext<'a, '_>,
    uncreate: ItemPtr,
) -> UncreateResult<'a> {
    Ok(
        if let Some(cshown) = env.get_and_downcast_construct_definition::<DShown>(uncreate)? {
            let cshown = cshown.clone();
            Some(Node {
                phrase: "shown",
                children: vec![NodeChild::Node(env.vomit(4, ctx, cshown.get_base()))],
                ..Default::default()
            })
        } else {
            None
        },
    )
}

fn vomit(pc: &ParseContext, src: &Node) -> String {
    format!("{}.SHOWN", src.children[0].vomit(pc))
}

pub fn phrase() -> Phrase {
    phrase!(
        "shown",
        128, 128,
        Some((create, uncreate)),
        vomit,
        4 => 4, r"\.SHOWN"
    )
}
