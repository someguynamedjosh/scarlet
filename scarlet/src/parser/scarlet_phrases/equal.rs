use typed_arena::Arena;

use crate::{
    environment::{vomit::VomitContext, Environment},
    item::{definitions::decision::DDecision, equality::Equal, Item, ItemDefinition, ItemPtr},
    parser::{
        phrase::{Phrase, UncreateResult},
        Node, NodeChild, ParseContext,
    },
    phrase,
    scope::{SPlain, Scope},
};

fn create(pc: &ParseContext, env: &mut Environment, scope: Box<dyn Scope>, node: &Node) -> ItemPtr {
    assert_eq!(node.children.len(), 3);
    assert_eq!(node.children[1], NodeChild::Text("="));
    let this = Item::placeholder_with_scope(scope);

    let left = node.children[0].as_construct(pc, env, SPlain(this.ptr_clone()));
    let right = node.children[2].as_construct(pc, env, SPlain(this.ptr_clone()));
    let truee = env.get_language_item("true").ptr_clone();
    let falsee = env.get_language_item("false").ptr_clone();
    this.redefine(DDecision::new(left, right, truee, falsee).clone_into_box());
    this
}

fn uncreate<'a>(
    env: &mut Environment,
    ctx: &mut VomitContext<'a, '_>,
    uncreate: ItemPtr,
) -> UncreateResult<'a> {
    Ok(
        if let Some(cite) = uncreate.downcast_definition::<DDecision>() {
            let cite = cite.clone();
            let truee = env.get_language_item("true");
            let falsee = env.get_language_item("false");
            if cite.when_equal().get_equality(&truee, 1024)? == Equal::yes()
                && cite.when_not_equal().get_equality(&falsee, 1024)? == Equal::yes()
            {
                Some(Node {
                    phrase: "equal",
                    children: vec![
                        NodeChild::Node(env.vomit(127, ctx, cite.left().ptr_clone())),
                        NodeChild::Text("="),
                        NodeChild::Node(env.vomit(127, ctx, cite.right().ptr_clone())),
                    ],
                    ..Default::default()
                })
            } else {
                None
            }
        } else {
            None
        },
    )
}

fn vomit(pc: &ParseContext, src: &Node) -> String {
    format!(
        "{} = {}",
        src.children[0].as_node().vomit(pc),
        src.children[2].as_node().vomit(pc)
    )
}

pub fn phrase() -> Phrase {
    phrase!(
        "equal",
        120, 120,
        Some((create, uncreate)),
        vomit,
        128 => 127, r"=", 127
    )
}
