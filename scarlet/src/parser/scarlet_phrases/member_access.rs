use typed_arena::Arena;

use crate::{
    constructs::ItemId,
    environment::{vomit::VomitContext, Environment},
    parser::{
        phrase::{Phrase, UncreateResult},
        Node, ParseContext,
    },
    phrase,
    resolvable::RNamedMember,
    scope::Scope,
};

fn create<'x>(
    pc: &ParseContext,
    env: &mut Environment<'x>,
    scope: Box<dyn Scope>,
    node: &Node<'x>,
) -> ItemId {
    let base = node.children[0].as_construct_dyn_scope(pc, env, scope.dyn_clone());
    let member_name = node.children[2].as_node();
    if member_name.phrase != "identifier" {
        todo!("Nice error, member access expected an identifier.");
    }
    let member_name = member_name.children[0].as_text();
    env.push_unresolved(RNamedMember { base, member_name }, scope)
}

fn uncreate<'a>(
    env: &mut Environment,
    ctx: &mut VomitContext<'a, '_>,
    uncreate: ItemId,
) -> UncreateResult<'a> {
    Ok(None)
}

fn vomit(_pc: &ParseContext, src: &Node) -> String {
    format!("{:#?}", src)
}

pub fn phrase() -> Phrase {
    phrase!(
        "member access",
        148, 128,
        Some((create, uncreate)),
        vomit,
        4 => 4, r"\.", 4
    )
}
