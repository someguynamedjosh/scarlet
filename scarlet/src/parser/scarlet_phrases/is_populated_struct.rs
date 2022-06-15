use typed_arena::Arena;

use crate::{
    environment::{vomit::VomitContext, Environment},
    item::{
        definitions::{is_populated_struct::DIsPopulatedStruct, other::DOther},
        Item, ItemDefinition, ItemPtr,
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
    assert_eq!(node.children[1], NodeChild::Text(".IS_POPULATED_STRUCT"));
    let this = Item::placeholder_with_scope(scope.dyn_clone());
    let base = node.children[0].as_construct(pc, env, SPlain(this.ptr_clone()));
    let ips = DIsPopulatedStruct::new(env, base, scope);
    this.redefine(DOther::new(ips).clone_into_box());
    this
}

fn uncreate<'a>(
    env: &mut Environment,
    ctx: &mut VomitContext<'a, '_>,
    uncreate: ItemPtr,
) -> UncreateResult<'a> {
    Ok(
        if let Some(cips) = uncreate.downcast_definition::<DIsPopulatedStruct>() {
            Some(Node {
                phrase: "is populated struct",
                children: vec![NodeChild::Node(env.vomit(
                    4,
                    ctx,
                    cips.get_base().ptr_clone(),
                ))],
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
