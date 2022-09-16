use crate::{
    diagnostic::Diagnostic,
    environment::{vomit::VomitContext, Environment},
    item::{definitions::axiom::DAxiom, Item, ItemDefinition, ItemPtr},
    parser::{
        phrase::{Phrase, UncreateResult},
        util::collect_comma_list,
        Node, NodeChild, ParseContext,
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
    assert_eq!(node.children.len(), 4);
    assert_eq!(node.children[0], NodeChild::Text("AXIOM"));
    assert_eq!(node.children[1], NodeChild::Text("("));
    assert_eq!(node.children[3], NodeChild::Text(")"));
    let settings = collect_comma_list(&node.children[2]);
    let name = settings[0].as_ident()?;
    let mut relying_on = Vec::new();
    if settings.len() > 1 {
        assert_eq!(settings[1].as_ident()?, "DEPENDING_ON");
        for child in &settings[2..] {
            relying_on.push(child.as_item_dyn_scope(pc, env, scope.dyn_clone())?);
        }
    }
    let con = DAxiom::from_name(env, name, relying_on).ok_or_else(|| {
        Diagnostic::new()
            .with_text_error(format!("{} is not a valid axiom:", name))
            .with_source_code_block_error(settings[0].position)
    })?;
    Ok(Item::new_boxed(con.clone_into_box(), scope))
}

fn uncreate<'a>(
    env: &mut Environment,
    _ctx: &mut VomitContext<'a, '_>,
    uncreate: ItemPtr,
) -> UncreateResult<'a> {
    if let Some(cax) = uncreate.downcast_definition::<DAxiom>() {
        let cax = cax.clone();
        let statement = cax.get_statement(env);
        let statement = &statement[..statement.len() - "_statement".len()];
        Ok(Some(Node {
            phrase: "axiom",
            children: vec![
                NodeChild::Text("AXIOM"),
                NodeChild::Text("("),
                NodeChild::Text(statement),
                NodeChild::Text(")"),
            ],
            ..Default::default()
        }))
    } else {
        Ok(None)
    }
}

fn vomit(_pc: &ParseContext, src: &Node) -> String {
    format!("AXIOM({})", src.children[2].as_text())
}

pub fn phrase() -> Phrase {
    phrase!(
        "axiom",
        128, 128,
        Some((create, uncreate)),
        vomit,
        4 => r"\bAXIOM\b", r"\(", 255, r"\)"
    )
}
