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
        "new type",
        128,
        Some((create,)),
        4 => "NEW_TYPE", r"\(", 255, r"\)"
    )
}
