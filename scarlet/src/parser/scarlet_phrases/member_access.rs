use crate::{
    diagnostic::Diagnostic,
    environment::{vomit::VomitContext, Environment},
    item::{
        resolvable::{DResolvable, RNamedMember},
        Item, ItemDefinition, ItemPtr,
    },
    parser::{
        phrase::{Phrase, UncreateResult},
        Node, ParseContext,
    },
    phrase,
    scope::Scope,
};

fn create(
    pc: &ParseContext,
    env: &mut Environment,
    scope: Box<dyn Scope>,
    node: &Node,
) -> Result<ItemPtr, Diagnostic> {
    let base = node.children[0].as_construct_dyn_scope(pc, env, scope.dyn_clone())?;
    let member_name = node.children[2].as_node();
    let position = member_name.position;
    let member_name = member_name.as_ident()?.to_owned();
    Ok(Item::new_boxed(
        DResolvable::new(RNamedMember {
            base,
            member_name,
            position,
        })
        .clone_into_box(),
        scope,
    ))
}

fn uncreate<'a>(
    _env: &mut Environment,
    _ctx: &mut VomitContext<'a, '_>,
    _uncreate: ItemPtr,
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
