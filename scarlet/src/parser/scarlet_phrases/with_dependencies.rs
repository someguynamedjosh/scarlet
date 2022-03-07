use itertools::Itertools;
use typed_arena::Arena;

use crate::{
    constructs::{with_dependencies::CWithDependencies, ItemId},
    environment::{vomit::VomitContext, Environment},
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
    assert_eq!(node.children[2], NodeChild::Text("["));
    assert_eq!(node.children[4], NodeChild::Text("]"));
    assert!(node.children.len() == 5);
    let this = env.push_placeholder(scope);
    let base = node.children[0].as_construct(pc, env, SPlain(this));
    let deps = util::collect_comma_list(&node.children[3])
        .iter()
        .map(|c| c.as_construct(pc, env, SPlain(this)))
        .collect();
    env.define_item(this, CWithDependencies::new(base, deps));
    this
}

fn uncreate<'a>(
    env: &mut Environment,
    ctx: &mut VomitContext<'a, '_>,
    uncreate: ItemId,
) -> UncreateResult<'a> {
    if let Ok(Some(cwd)) = env.get_and_downcast_construct_definition::<CWithDependencies>(uncreate)
    {
        let cwd = cwd.clone();
        let deps = create_comma_list(
            cwd.dependencies()
                .into_iter()
                .map(|dep| env.vomit(254, ctx, *dep))
                .collect_vec(),
        );
        Ok(Some(Node {
            phrase: "with dependencies",
            children: vec![
                NodeChild::Node(env.vomit(4, ctx, cwd.base())),
                NodeChild::Text("."),
                NodeChild::Text("WITH_DEPENDENCIES"),
                NodeChild::Text("["),
                deps,
                NodeChild::Text("]"),
            ],
        }))
    } else {
        Ok(None)
    }
}

fn vomit(pc: &ParseContext, src: &Node) -> String {
    format!(
        "{}.WITH_DEPENDENCIES[ {} ]",
        src.children[0].vomit(pc),
        src.children[4].vomit(pc)
    )
}

pub fn phrase() -> Phrase {
    phrase!(
        "with dependencies",
        128, 128,
        Some((create, uncreate)),
        vomit,
        4 => 4, r"\.WITH_DEPENDENCIES", r"\[", 255, r"\]"
    )
}
