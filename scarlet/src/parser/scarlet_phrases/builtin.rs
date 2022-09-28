use crate::{
    parser::{
        phrase::{CreateContext, CreateResult, Phrase},
        Node,
    },
    phrase,
    scope::Scope,
};

pub fn create(ctx: &mut CreateContext, scope: Box<dyn Scope>, node: &Node) -> CreateResult {
    todo!()
}

pub fn phrase() -> Phrase {
    phrase!(
        "builtin",
        128,
        Some((create,)),
        4 => "BUILTIN", r"\(", 255, r"\)"
    )
}
