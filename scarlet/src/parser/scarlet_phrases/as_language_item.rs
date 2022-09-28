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
        "as language item",
        128,
        Some((create,)),
        236 => 236, r"AS_LANGUAGE_ITEM", r"\(", 255, r"\)"
    )
}
