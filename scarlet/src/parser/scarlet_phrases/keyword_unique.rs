use typed_arena::Arena;

use crate::{
    constructs::{downcast_construct, unique::CUnique, ConstructId},
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
    pc: &ParseContext,
    env: &mut Environment,
    code_arena: &'a Arena<String>,
    uncreate: ConstructId,
    from: &dyn Scope,
) -> UncreateResult<'a> {
    Ok(
        if let Some(unique) = env.get_and_downcast_construct_definition::<CUnique>(uncreate)? {
            Some(Node {
                phrase: "keyword UNIQUE",
                children: vec![NodeChild::Text("UNIQUE")],
            })
        } else {
            None
        },
    )
}

fn vomit(_pc: &ParseContext, src: &Node) -> String {
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
