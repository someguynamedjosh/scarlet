use typed_arena::Arena;

use crate::{
    constructs::ItemId,
    environment::{vomit::VomitContext, Environment},
    parser::{
        phrase::{Phrase, UncreateResult},
        Node, NodeChild, ParseContext,
    },
    phrase,
    resolvable::RIdentifier,
    scope::Scope,
};

fn create<'x>(
    _pc: &ParseContext,
    env: &mut Environment<'x>,
    scope: Box<dyn Scope>,
    node: &Node<'x>,
) -> ItemId {
    assert_eq!(node.phrase, "identifier");
    assert_eq!(node.children.len(), 1);
    env.push_unresolved(RIdentifier(node.children[0].as_text()), scope)
}

fn uncreate<'a>(
    env: &mut Environment,
    ctx: &mut VomitContext<'a, '_>,
    uncreate: ItemId,
) -> UncreateResult<'a> {
    let dereffed = env.dereference(uncreate)?;
    Ok(if dereffed == uncreate {
        None
    } else if let Ok(Some(ident)) = ctx.scope.reverse_lookup_ident(env, dereffed) {
        Some(Node {
            phrase: "identifier",
            children: vec![NodeChild::Text(ctx.code_arena.alloc(ident))],
        })
    } else {
        None
    })
}

fn vomit(_pc: &ParseContext, src: &Node) -> String {
    format!("{}", src.children[0].as_text())
}

pub fn phrase() -> Phrase {
    phrase!(
        "identifier",
        255, 0,
        Some((create, uncreate)),
        vomit,
        0 => r"[a-zA-Z0-9_]+"
    )
}
