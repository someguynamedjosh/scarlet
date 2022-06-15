use crate::{
    environment::{vomit::VomitContext, Environment},
    item::ItemPtr,
    parser::{
        phrase::{Phrase, UncreateResult},
        Node, ParseContext,
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
    _env: &mut Environment,
    _ctx: &mut VomitContext<'x, '_>,
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

fn vomit(_pc: &ParseContext, _src: &Node) -> String {
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
