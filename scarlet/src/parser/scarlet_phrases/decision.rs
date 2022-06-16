use maplit::hashset;

use crate::{
    diagnostic::Diagnostic,
    environment::{vomit::VomitContext, Environment},
    item::{
        definitions::decision::{DDecision, SWithInvariant},
        invariants::InvariantSet,
        Item, ItemDefinition, ItemPtr,
    },
    parser::{
        phrase::{Phrase, UncreateResult},
        util::{self, create_comma_list},
        Node, NodeChild, ParseContext,
    },
    phrase,
    scope::{SPlain, Scope},
};

fn create(
    pc: &ParseContext,
    env: &mut Environment,
    scope: Box<dyn Scope>,
    node: &Node,
) -> Result<ItemPtr, Diagnostic> {
    assert_eq!(node.children.len(), 4);
    assert_eq!(node.children[0], NodeChild::Text("DECISION"));
    assert_eq!(node.children[1], NodeChild::Text("("));
    assert_eq!(node.children[3], NodeChild::Text(")"));
    let args = util::collect_comma_list(&node.children[2]);
    assert_eq!(args.len(), 4);
    let this = Item::placeholder_with_scope(scope);

    let truee = env.get_language_item("true").unwrap().ptr_clone();
    let falsee = env.get_language_item("false").unwrap().ptr_clone();
    let left = args[0].as_item(pc, env, SPlain(this.ptr_clone()))?;
    let right = args[1].as_item(pc, env, SPlain(this.ptr_clone()))?;

    let eq_inv = Item::new(
        DDecision::new(
            left.ptr_clone(),
            right.ptr_clone(),
            truee.ptr_clone(),
            falsee.ptr_clone(),
        ),
        SPlain(this.ptr_clone()),
    );
    let eq_inv = InvariantSet::new_root_statements_depending_on(
        this.ptr_clone(),
        vec![eq_inv],
        hashset![this.ptr_clone()],
    );
    let equal = args[2].as_item(pc, env, SWithInvariant(eq_inv, this.ptr_clone()))?;

    let neq_inv = Item::new(
        DDecision::new(
            left.ptr_clone(),
            right.ptr_clone(),
            falsee.ptr_clone(),
            truee.ptr_clone(),
        ),
        SPlain(this.ptr_clone()),
    );
    let neq_inv = InvariantSet::new_root_statements_depending_on(
        this.ptr_clone(),
        vec![neq_inv],
        hashset![this.ptr_clone()],
    );
    let unequal = args[3].as_item(pc, env, SWithInvariant(neq_inv, this.ptr_clone()))?;

    this.redefine(
        DDecision::new(left.ptr_clone(), right.ptr_clone(), equal, unequal).clone_into_box(),
    );

    Ok(this)
}

fn uncreate<'a>(
    env: &mut Environment,
    ctx: &mut VomitContext<'a, '_>,
    uncreate: ItemPtr,
) -> UncreateResult<'a> {
    if let Some(cite) = uncreate.downcast_definition::<DDecision>() {
        let cite = cite.clone();
        Ok(Some(Node {
            phrase: "decision",
            children: vec![
                NodeChild::Text("DECISION"),
                NodeChild::Text("("),
                create_comma_list(vec![
                    env.vomit(255, ctx, cite.left().ptr_clone()),
                    env.vomit(255, ctx, cite.right().ptr_clone()),
                    env.vomit(255, ctx, cite.when_equal().ptr_clone()),
                    env.vomit(255, ctx, cite.when_not_equal().ptr_clone()),
                ]),
                NodeChild::Text(")"),
            ],
            ..Default::default()
        }))
    } else {
        Ok(None)
    }
}

fn vomit(pc: &ParseContext, src: &Node) -> String {
    format!("DECISION[ {} ]", src.children[2].as_node().vomit(pc))
}

pub fn phrase() -> Phrase {
    phrase!(
        "decision",
        128, 128,
        Some((create, uncreate)),
        vomit,
        0 => r"\bDECISION\b" , r"\(", 255, r"\)"
    )
}
