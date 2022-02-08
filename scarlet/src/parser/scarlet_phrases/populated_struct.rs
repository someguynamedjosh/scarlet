use typed_arena::Arena;

use crate::{
    constructs::{
        structt::{CPopulatedStruct, SField, SFieldAndRest},
        ConstructId,
    },
    environment::Environment,
    parser::{phrase::{Phrase, UncreateResult}, util, Node, NodeChild, ParseContext},
    phrase,
    scope::Scope,
};

fn create<'x>(
    pc: &ParseContext,
    env: &mut Environment<'x>,
    scope: Box<dyn Scope>,
    node: &Node<'x>,
) -> ConstructId {
    assert!(node.children.len() == 4);
    assert_eq!(node.children[0], NodeChild::Text("POPULATED_STRUCT"));
    assert_eq!(node.children[1], NodeChild::Text("["));
    assert_eq!(node.children[3], NodeChild::Text("]"));
    let args = util::collect_comma_list(&node.children[2]);
    assert_eq!(args.len(), 3);
    let this = env.push_placeholder(scope);

    let label = args[0].as_ident().to_owned();
    let value = args[1].as_construct(pc, env, SFieldAndRest(this));
    let rest = args[2].as_construct(pc, env, SField(this));
    env.define_construct(this, CPopulatedStruct::new(label, value, rest));
    this
}

fn uncreate<'a>(
    _pc: &ParseContext,
    _env: &mut Environment,
    _code_arena: &'a Arena<String>,
    _uncreate: ConstructId,
    _from: &dyn Scope,
) -> UncreateResult<'a> {
    Ok(None)
}

fn vomit(_pc: &ParseContext, src: &Node) -> String {
    format!("{:#?}", src)
}

pub fn phrase() -> Phrase {
    phrase!(
        "populated struct",
        128, 128,
        Some((create, uncreate)),
        vomit,
        0 => r"\bPOPULATED_STRUCT\b" , r"\[", 255, r"\]"
    )
}
