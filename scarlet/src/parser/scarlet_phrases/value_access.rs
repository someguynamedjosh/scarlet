use std::ops::ControlFlow;

use crate::{
    constructs::{
        structt::{AtomicStructMember, CAtomicStructMember, CPopulatedStruct},
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
    env.define_item(this, CAtomicStructMember(base, AtomicStructMember::Value));
    this
}

fn uncreate<'a>(
    env: &mut Environment,
    ctx: &mut VomitContext<'a, '_>,
    uncreate: ItemId,
) -> UncreateResult<'a> {
    let source = if let Some(asm) =
        env.get_and_downcast_construct_definition::<CAtomicStructMember>(uncreate)?
    {
        if asm.1 == AtomicStructMember::Value {
            Some(asm.0)
        } else {
            None
        }
    } else {
        env.for_each_item(|env, id| {
            if let Ok(Some(cstruct)) =
                env.get_and_downcast_construct_definition::<CPopulatedStruct>(id)
            {
                let cstruct = cstruct.clone();
                if cstruct.get_value() == uncreate && ctx.scope.parent() != Some(id) {
                    return ControlFlow::Break(id);
                }
            }
            ControlFlow::Continue(())
        })
    };
    Ok(if let Some(id) = source {
        Some(Node {
            phrase: "value access",
            children: vec![
                NodeChild::Node(env.vomit(4, ctx, id)),
                NodeChild::Text(".VALUE"),
            ],
            ..Default::default()
        })
    } else {
        None
    })
}

fn vomit(pc: &ParseContext, src: &Node) -> String {
    format!("{}.VALUE", src.children[0].as_node().vomit(pc))
}

pub fn phrase() -> Phrase {
    phrase!(
        "value access",
        128, 136,
        Some((create, uncreate)),
        vomit,
        4 => 4, r"\.VALUE"
    )
}
