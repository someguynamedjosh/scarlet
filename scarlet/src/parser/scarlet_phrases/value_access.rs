use std::ops::ControlFlow;

use typed_arena::Arena;

use crate::{
    environment::{vomit::VomitContext, Environment},
    item::{
        definitions::structt::{AtomicStructMember, DAtomicStructMember, DPopulatedStruct},
        equality::Equal,
        ItemDefinition, ItemPtr,
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
    let this = crate::item::Item::placeholder_with_scope(scope);
    let base = node.children[0].as_construct(pc, env, SPlain(this.ptr_clone()));
    this.redefine(DAtomicStructMember::new(base, AtomicStructMember::Value).clone_into_box());
    this
}

fn uncreate<'a>(
    env: &mut Environment,
    ctx: &mut VomitContext<'a, '_>,
    uncreate: ItemPtr,
) -> UncreateResult<'a> {
    let source = if let Some(asm) = uncreate.downcast_definition::<DAtomicStructMember>() {
        if asm.member() == AtomicStructMember::Value {
            Some(asm.base().ptr_clone())
        } else {
            None
        }
    } else {
        None
    };
    Ok(if let Some(id) = source {
        Some(Node {
            phrase: "value access",
            children: vec![
                NodeChild::Node(env.vomit(4, ctx, id.ptr_clone())),
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
