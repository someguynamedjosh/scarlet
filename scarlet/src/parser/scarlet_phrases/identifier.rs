use typed_arena::Arena;

use crate::{
    environment::{vomit::VomitContext, Environment},
    item::{resolvable::RIdentifier, ItemPtr},
    parser::{
        phrase::{Phrase, UncreateResult},
        Node, NodeChild, ParseContext,
    },
    phrase,
    scope::Scope,
};

fn create(
    _pc: &ParseContext,
    env: &mut Environment,
    scope: Box<dyn Scope>,
    node: &Node,
) -> ItemPtr {
    assert_eq!(node.phrase, "identifier");
    assert_eq!(node.children.len(), 1);
    env.push_unresolved(RIdentifier(node.children[0].as_text()), scope)
}

fn uncreate<'a>(
    env: &mut Environment,
    ctx: &mut VomitContext<'a, '_>,
    uncreate: ItemPtr,
) -> UncreateResult<'a> {
    let dereffed = env.dereference(uncreate)?;
    Ok(if dereffed == uncreate {
        None
    } else if let Ok(Some(ident)) = ctx.scope.reverse_lookup_ident(env, dereffed) {
        Some(Node {
            phrase: "identifier",
            children: vec![NodeChild::Text(ctx.code_arena.alloc(ident))],
            ..Default::default()
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
