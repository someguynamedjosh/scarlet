use typed_arena::Arena;

use crate::{
    constructs::{
        decision::{CDecision, SWithInvariant},
        ItemId,
    },
    environment::{invariants::Invariant, vomit::VomitContext, Environment},
    parser::{
        phrase::{Phrase, UncreateResult},
        util::{self, create_comma_list},
        Node, NodeChild, ParseContext,
    },
    phrase,
    scope::{SPlain, Scope},
};

fn create<'x>(
    pc: &ParseContext,
    env: &mut Environment<'x>,
    scope: Box<dyn Scope>,
    node: &Node<'x>,
) -> ItemId {
    assert_eq!(node.children.len(), 4);
    assert_eq!(node.children[0], NodeChild::Text("DECISION"));
    assert_eq!(node.children[1], NodeChild::Text("["));
    assert_eq!(node.children[3], NodeChild::Text("]"));
    let args = util::collect_comma_list(&node.children[2]);
    assert_eq!(args.len(), 4);
    let this = env.push_placeholder(scope);

    let truee = env.get_language_item("true");
    let falsee = env.get_language_item("false");
    let left = args[0].as_construct(pc, env, SPlain(this));
    let right = args[1].as_construct(pc, env, SPlain(this));

    let eq_inv = env.push_construct(
        CDecision::new(left, right, truee, falsee),
        SPlain(this).dyn_clone(),
    );
    let eq_inv = Invariant::new(eq_inv, Default::default());
    let equal = args[2].as_construct(pc, env, SWithInvariant(eq_inv, this));

    let neq_inv = env.push_construct(
        CDecision::new(left, right, falsee, truee),
        SPlain(this).dyn_clone(),
    );
    let neq_inv = Invariant::new(neq_inv, Default::default());
    let unequal = args[3].as_construct(pc, env, SWithInvariant(neq_inv, this));

    env.define_item(this, CDecision::new(left, right, equal, unequal));
    this
}

fn uncreate<'a>(
    env: &mut Environment,
    ctx: &mut VomitContext<'a, '_>,
    uncreate: ItemId,
) -> UncreateResult<'a> {
    if let Some(cite) = env.get_and_downcast_construct_definition::<CDecision>(uncreate)? {
        let cite = cite.clone();
        Ok(Some(Node {
            phrase: "decision",
            children: vec![
                NodeChild::Text("DECISION"),
                NodeChild::Text("["),
                create_comma_list(vec![
                    env.vomit(255, ctx, cite.left()),
                    env.vomit(255, ctx, cite.right()),
                    env.vomit(255, ctx, cite.equal()),
                    env.vomit(255, ctx, cite.unequal()),
                ]),
                NodeChild::Text("]"),
            ],
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
        0 => r"\bDECISION\b" , r"\[", 255, r"\]"
    )
}
