use typed_arena::Arena;

use crate::{
    environment::{vomit::VomitContext, Environment},
    item::{definitions::other::DOther, ItemPtr},
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

fn uncreate<'x>(
    env: &mut Environment,
    ctx: &mut VomitContext<'x, '_>,
    uncreate: ItemPtr,
) -> UncreateResult<'x> {
    if !uncreate.is_recursive() {
        Ok(None)
    } else {
        Ok(Some(Node {
            phrase: "recursion",
            // children: vec![NodeChild::Node(env.vomit(255, ctx, base))],
            children: vec![],
            ..Default::default()
        }))
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
