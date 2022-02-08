use typed_arena::Arena;

use crate::{
    constructs::ConstructId,
    environment::Environment,
    parser::{phrase::{Phrase, UncreateResult}, Node, ParseContext},
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
) -> UncreateResult<'a> {
    Ok(None)
}

fn vomit(_pc: &ParseContext, src: &Node) -> String {
    format!("{:#?}", src)
}

pub fn phrase() -> Phrase {
    phrase!(
        "member access",
        128, 128,
        Some((create, uncreate)),
        vomit,
        4 => 4, r"\.", 4
    )
}
