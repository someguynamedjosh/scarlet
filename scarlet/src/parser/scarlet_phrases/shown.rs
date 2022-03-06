use typed_arena::Arena;

use crate::{
    constructs::{shown::CShown, ItemId},
    environment::Environment,
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
    assert_eq!(node.children[1], NodeChild::Text(".SHOWN"));
    let this = env.push_placeholder(scope);
    let base = node.children[0].as_construct(pc, env, SPlain(this));
    env.define_item(this, CShown::new(base));
    this
}

fn uncreate<'a>(
    pc: &ParseContext,
    env: &mut Environment,
    code_arena: &'a Arena<String>,
    uncreate: ItemId,
    from: &dyn Scope,
) -> UncreateResult<'a> {
    Ok(
        if let Some(cshown) = env.get_and_downcast_construct_definition::<CShown>(uncreate)? {
            let cshown = cshown.clone();
            Some(Node {
                phrase: "shown",
                children: vec![NodeChild::Node(env.vomit(
                    4,
                    pc,
                    code_arena,
                    cshown.get_base(),
                    from,
                )?)],
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
