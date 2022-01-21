use itertools::Itertools;
use typed_arena::Arena;

use crate::{
    constructs::{
        downcast_construct, substitution::CSubstitution, with_dependencies::CWithDependencies,
        ConstructId,
    },
    environment::Environment,
    parser::{
        phrase::Phrase,
        util::{self, create_comma_list},
        Node, NodeChild, ParseContext,
    },
    phrase,
    resolvable::RSubstitution,
    scope::{SPlain, Scope},
};

fn create<'x>(
    pc: &ParseContext,
    env: &mut Environment<'x>,
    scope: Box<dyn Scope>,
    node: &Node<'x>,
) -> ConstructId {
    assert_eq!(node.children[3], NodeChild::Text("["));
    assert_eq!(node.children[5], NodeChild::Text("]"));
    assert!(node.children.len() == 6);
    let this = env.push_placeholder(scope);
    let base = node.children[0].as_construct(pc, env, SPlain(this));
    let deps = util::collect_comma_list(&node.children[4])
        .iter()
        .map(|c| c.as_construct(pc, env, SPlain(this)))
        .collect();
    env.define_construct(this, CWithDependencies::new(base, deps));
    this
}

fn uncreate<'a>(
    pc: &ParseContext,
    env: &mut Environment,
    code_arena: &'a Arena<String>,
    uncreate: ConstructId,
    from: &dyn Scope,
) -> Option<Node<'a>> {
    if let Some(cwd) = env.get_construct_definition_for_vomiting::<CWithDependencies>(uncreate) {
        let cwd = cwd.clone();
        let deps = create_comma_list(
            cwd.dependencies()
                .into_iter()
                .map(|dep| env.vomit(254, pc, code_arena, *dep, from))
                .collect_vec(),
        );
        Some(Node {
            phrase: "with dependencies",
            children: vec![
                NodeChild::Node(env.vomit(4, pc, code_arena, cwd.base(), from)),
                NodeChild::Text("."),
                NodeChild::Text("WITH_DEPENDENCIES"),
                NodeChild::Text("["),
                deps,
                NodeChild::Text("]"),
            ],
        })
    } else {
        None
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
        4 => 4, r"\.", r"WITH_DEPENDENCIES", r"\[", 255, r"\]"
    )
}
