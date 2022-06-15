use typed_arena::Arena;

use crate::{
    environment::{vomit::VomitContext, Environment},
    item::{
        definitions::structt::{AtomicStructMember, DAtomicStructMember},
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
    this.redefine(DAtomicStructMember::new(base, AtomicStructMember::Label).clone_into_box());
    this
}

fn uncreate<'a>(
    env: &mut Environment,
    ctx: &mut VomitContext<'a, '_>,
    uncreate: ItemPtr,
) -> UncreateResult<'a> {
    Ok(
        if let Some(asm) = uncreate.downcast_definition::<DAtomicStructMember>() {
            if asm.member() == AtomicStructMember::Label {
                let id = asm.base();
                Some(Node {
                    phrase: "label access",
                    children: vec![
                        NodeChild::Node(env.vomit(4, ctx, id.ptr_clone())),
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
