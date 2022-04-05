use crate::{
    constructs::{
        structt::{AtomicStructMember, CAtomicStructMember},
        ItemId,
    },
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
    let this = env.push_placeholder(scope);
    let base = node.children[0].as_construct(pc, env, SPlain(this));
    env.define_item(this, CAtomicStructMember(base, AtomicStructMember::Label));
    this
}

fn uncreate<'a>(
    env: &mut Environment,
    ctx: &mut VomitContext<'a, '_>,
    uncreate: ItemId,
) -> UncreateResult<'a> {
    Ok(
        if let Some(asm) =
            env.get_and_downcast_construct_definition::<CAtomicStructMember>(uncreate)?
        {
            if asm.1 == AtomicStructMember::Label {
                let id = asm.0;
                Some(Node {
                    phrase: "label access",
                    children: vec![
                        NodeChild::Node(env.vomit(4, ctx, id)),
                        NodeChild::Text(".LABEL"),
                    ],
                    ..Default::default()
                })
            } else {
                None
            }
        } else {
            None
        },
    )
}

fn vomit(_pc: &ParseContext, src: &Node) -> String {
    format!("{:#?}", src)
}

pub fn phrase() -> Phrase {
    phrase!(
        "label access",
        128, 128,
        Some((create, uncreate)),
        vomit,
        4 => 4, r"\.LABEL"
    )
}
