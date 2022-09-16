use crate::{
    diagnostic::Diagnostic,
    environment::{vomit::VomitContext, Environment},
    item::{
        definitions::{axiom::DAxiom, builtin_function::DBuiltinFunction},
        Item, ItemDefinition, ItemPtr,
    },
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
) -> Result<ItemPtr, Diagnostic> {
    assert_eq!(node.children.len(), 4);
    assert_eq!(node.children[0], NodeChild::Text("BUILTIN_FUNCTION"));
    assert_eq!(node.children[1], NodeChild::Text("("));
    assert_eq!(node.children[3], NodeChild::Text(")"));
    let name_node = node.children[2].as_node();
    let name = name_node.as_ident()?;
    let con = DBuiltinFunction::from_name(env, name).ok_or_else(|| {
        Diagnostic::new()
            .with_text_error(format!("{} is not a valid builtin function:", name))
            .with_source_code_block_error(name_node.position)
    })?;
    Ok(Item::new_boxed(con.clone_into_box(), scope))
}

fn uncreate<'a>(
    env: &mut Environment,
    _ctx: &mut VomitContext<'a, '_>,
    uncreate: ItemPtr,
) -> UncreateResult<'a> {
    if let Some(cfn) = uncreate.downcast_definition::<DBuiltinFunction>() {
        let cfn = cfn.clone();
        let name = cfn.get_name();
        Ok(Some(Node {
            phrase: "builtin function",
            children: vec![
                NodeChild::Text("BUILTIN_FUNCTION"),
                NodeChild::Text("("),
                NodeChild::Text(name),
                NodeChild::Text(")"),
            ],
            ..Default::default()
        }))
    } else {
        Ok(None)
    }
}

fn vomit(_pc: &ParseContext, src: &Node) -> String {
    format!("BUILTIN_FUNCTION({})", src.children[2].as_text())
}

pub fn phrase() -> Phrase {
    phrase!(
        "builtin function",
        128, 128,
        Some((create, uncreate)),
        vomit,
        4 => r"\bBUILTIN_FUNCTION\b", r"\(", 255, r"\)"
    )
}
