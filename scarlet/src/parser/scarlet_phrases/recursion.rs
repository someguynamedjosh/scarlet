use typed_arena::Arena;

use crate::{
    item::{recursion::CRecursion, ItemPtr},
    environment::{vomit::VomitContext, Environment},
    parser::{
        phrase::{Phrase, UncreateResult},
        Node, NodeChild, ParseContext,
    },
    phrase,
    scope::Scope,
};

fn create(
    _pc: &ParseContext,
    _env: &mut Environment,
    _scope: Box<dyn Scope>,
    _node: &Node,
) -> ItemPtr {
    unreachable!()
}

fn uncreate(
    env: &mut Environment,
    ctx: &mut VomitContext<'x, '_>,
    uncreate: ItemPtr,
) -> UncreateResult {
    if let Some(recursion) = env.get_and_downcast_construct_definition::<CRecursion>(uncreate)? {
        let base = recursion.get_base();
        Ok(Some(Node {
            phrase: "recursion",
            // children: vec![NodeChild::Node(env.vomit(255, ctx, base))],
            children: vec![],
            ..Default::default()
        }))
    } else {
        Ok(None)
    }
}

fn vomit(pc: &ParseContext, src: &Node) -> String {
    // format!("#= recursive =# {}", src.children[0].as_node().vomit(pc))
    format!("RECURSE")
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
