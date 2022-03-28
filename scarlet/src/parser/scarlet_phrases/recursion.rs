use typed_arena::Arena;

use crate::{
    constructs::{recursion::CRecursion, ItemId},
    environment::{vomit::VomitContext, Environment},
    parser::{
        phrase::{Phrase, UncreateResult},
        Node, NodeChild, ParseContext,
    },
    phrase,
    scope::Scope,
};

fn create<'x>(
    _pc: &ParseContext,
    _env: &mut Environment<'x>,
    _scope: Box<dyn Scope>,
    _node: &Node<'x>,
) -> ItemId {
    unreachable!()
}

fn uncreate<'x>(
    env: &mut Environment,
    ctx: &mut VomitContext<'x, '_>,
    uncreate: ItemId,
) -> UncreateResult<'x> {
    if let Some(recursion) = env.get_and_downcast_construct_definition::<CRecursion>(uncreate)? {
        let base = recursion.get_base();
        Ok(Some(Node {
            phrase: "recursion",
            children: vec![NodeChild::Node(env.vomit(255, ctx, base))],
            ..Default::default()
        }))
    } else {
        Ok(None)
    }
}

fn vomit(pc: &ParseContext, src: &Node) -> String {
    format!("#= recursive =# {}", src.children[0].as_node().vomit(pc))
}

pub fn phrase() -> Phrase {
    phrase!(
        "recursion",
        128, 128,
        Some((create, uncreate)),
        vomit,
        0 => r"a^"
    )
}
