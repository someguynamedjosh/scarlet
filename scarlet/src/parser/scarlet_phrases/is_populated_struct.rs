use typed_arena::Arena;

use crate::{
    constructs::{is_populated_struct::CIsPopulatedStruct, ItemId},
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
    assert_eq!(node.children[1], NodeChild::Text(".IS_POPULATED_STRUCT"));
    let this = env.push_placeholder(scope);
    let base = node.children[0].as_construct(pc, env, SPlain(this));
    env.define_item(this, CIsPopulatedStruct::new(base));
    this
}

fn uncreate<'a>(
    _pc: &ParseContext,
    _env: &mut Environment,
    _code_arena: &'a Arena<String>,
    _uncreate: ItemId,
    _from: &dyn Scope,
) -> UncreateResult<'a> {
    Ok(None)
}

fn vomit(_pc: &ParseContext, src: &Node) -> String {
    format!("{:#?}", src)
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
