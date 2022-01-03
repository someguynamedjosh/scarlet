use typed_arena::Arena;

use crate::{
    constructs::ConstructId,
    environment::Environment,
    parser::{phrase::Phrase, Node, ParseContext},
    phrase,
    scope::Scope,
};

fn create<'x>(
    _pc: &ParseContext,
    _env: &mut Environment<'x>,
    _scope: Box<dyn Scope>,
    _node: &Node<'x>,
) -> ConstructId {
    todo!()
}

fn uncreate<'a>(
    _pc: &ParseContext,
    _env: &mut Environment,
    _code_arena: &'a Arena<String>,
    _uncreate: ConstructId,
    _from: &dyn Scope,
) -> Option<Node<'a>> {
    None
}

fn vomit(_pc: &ParseContext, src: &Node) -> String {
    format!("{:#?}", src)
}

pub fn phrase() -> Phrase {
    phrase!(
        "keyword AXIOM_OF_EQUALITY",
        Some((create, uncreate)),
        vomit,
        0 => r"\bAXIOM_OF_EQUALITY\b"
    )
}
