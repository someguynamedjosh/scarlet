use typed_arena::Arena;

use crate::{
    constructs::{unique::CUnique, ConstructId},
    environment::Environment,
    parser::{
        phrase::{Phrase, UncreateResult},
        Node, NodeChild, ParseContext,
    },
    phrase,
    scope::Scope,
};

fn create<'x>(
    _pc: &ParseContext,
    env: &mut Environment<'x>,
    scope: Box<dyn Scope>,
    node: &Node<'x>,
) -> ConstructId {
    assert_eq!(node.children, &[NodeChild::Text("UNIQUE")]);
    let id = env.push_unique();
    env.push_construct(CUnique::new(id), scope)
}

fn uncreate<'a>(
    _pc: &ParseContext,
    env: &mut Environment,
    _code_arena: &'a Arena<String>,
    uncreate: ConstructId,
    _from: &dyn Scope,
) -> UncreateResult<'a> {
    Ok(
        if let Some(_unique) = env.get_and_downcast_construct_definition::<CUnique>(uncreate)? {
            Some(Node {
                phrase: "keyword UNIQUE",
                children: vec![NodeChild::Text("UNIQUE")],
            })
        } else {
            None
        },
    )
}

fn vomit(_pc: &ParseContext, _src: &Node) -> String {
    format!("UNIQUE")
}

pub fn phrase() -> Phrase {
    phrase!(
        "keyword UNIQUE",
        128, 128,
        Some((create, uncreate)),
        vomit,
        0 => r"\bUNIQUE\b"
    )
}
