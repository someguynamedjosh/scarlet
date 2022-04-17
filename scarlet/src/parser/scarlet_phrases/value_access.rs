use std::ops::ControlFlow;

use typed_arena::Arena;

use crate::{
    environment::{vomit::VomitContext, Environment},
    item::{
        definitions::structt::{AtomicStructMember, DAtomicStructMember, DPopulatedStruct},
        equality::Equal,
        ItemPtr,
    },
    parser::{
        phrase::{Phrase, UncreateResult},
        Node, NodeChild, ParseContext,
    },
    phrase,
    scope::{SPlain, Scope},
};

fn create(pc: &ParseContext, env: &mut Environment, scope: Box<dyn Scope>, node: &Node) -> ItemPtr {
    assert_eq!(node.children.len(), 2);
    let this = env.push_placeholder(scope);
    let base = node.children[0].as_construct(pc, env, SPlain(this));
    env.define_item(this, DAtomicStructMember(base, AtomicStructMember::Value));
    this
}

fn uncreate<'a>(
    env: &mut Environment,
    ctx: &mut VomitContext<'a, '_>,
    uncreate: ItemPtr,
) -> UncreateResult<'a> {
    let source = if let Some(asm) =
        env.get_and_downcast_construct_definition::<DAtomicStructMember>(uncreate)?
    {
        if asm.1 == AtomicStructMember::Value {
            Some(asm.0)
        } else {
            None
        }
    } else {
        env.for_each_item(|env, id| {
            if let Ok(Some(cstruct)) =
                env.get_and_downcast_construct_definition::<DPopulatedStruct>(id)
            {
                let cstruct = cstruct.clone();
                if env.discover_equal(cstruct.get_value(), uncreate, 1024) == Ok(Equal::yes())
                    && ctx.scope.parent() != Some(id)
                {
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
