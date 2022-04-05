use itertools::Itertools;

use crate::{
    constructs::{
        substitution::CSubstitution,
        variable::{Dependency, VariableId},
        ItemId,
    },
    environment::{vomit::VomitContext, Environment, UnresolvedItemError},
    parser::{
        phrase::{Phrase, UncreateResult},
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
) -> ItemId {
    assert_eq!(node.children[1], NodeChild::Text("["));
    assert_eq!(node.children[3], NodeChild::Text("]"));
    assert!(node.children.len() == 4);
    let this = env.push_placeholder(scope);
    let base = node.children[0].as_construct(pc, env, SPlain(this));
    let mut named_subs = Vec::new();
    let mut anonymous_subs = Vec::new();
    for sub in util::collect_comma_list(&node.children[2]) {
        if sub.phrase == "is" {
            named_subs.push((
                sub.children[0].as_node().as_ident(),
                sub.children[2].as_construct(pc, env, SPlain(this)),
            ));
        } else {
            anonymous_subs.push(sub.as_construct(pc, env, SPlain(this)));
        }
    }
    env.define_unresolved(
        this,
        RSubstitution {
            base,
            named_subs,
            anonymous_subs,
        },
    );
    this
}

fn uncreate_substitution<'a>(
    env: &mut Environment,
    ctx: &mut VomitContext<'a, '_>,
    target: VariableId,
    value: ItemId,
    deps: &mut Vec<Dependency>,
) -> Result<Node<'a>, UnresolvedItemError> {
    let value = env.vomit(254, ctx, value);
    Ok(if deps.get(0).map(|v| v.id == target) == Some(true) {
        deps.remove(0);
        value
    } else {
        Node {
            phrase: "is",
            children: vec![
                NodeChild::Node(env.vomit_var(ctx, target)),
                NodeChild::Text("IS"),
                NodeChild::Node(value),
            ],
            ..Default::default()
        }
    })
}

fn uncreate<'a>(
    env: &mut Environment,
    ctx: &mut VomitContext<'a, '_>,
    uncreate: ItemId,
) -> UncreateResult<'a> {
    if let Some(csub) = env.get_and_downcast_construct_definition::<CSubstitution>(uncreate)? {
        let csub = csub.clone();
        let mut deps = env
            .get_dependencies(csub.base())
            .into_variables()
            .collect_vec();
        let subs = create_comma_list(
            csub.substitutions()
                .into_iter()
                .map(|(target, value)| uncreate_substitution(env, ctx, *target, *value, &mut deps))
                .collect::<Result<Vec<_>, _>>()?,
        );
        Ok(Some(Node {
            phrase: "substitution",
            children: vec![
                NodeChild::Node(env.vomit(4, ctx, csub.base())),
                NodeChild::Text("["),
                subs,
                NodeChild::Text("]"),
            ],
            ..Default::default()
        }))
    } else {
        Ok(None)
    }
}

fn vomit(pc: &ParseContext, src: &Node) -> String {
    format!(
        "{}[ {} ]",
        src.children[0].vomit(pc),
        src.children[2].vomit(pc)
    )
}

pub fn phrase() -> Phrase {
    phrase!(
        "substitution",
        128, 120,
        Some((create, uncreate)),
        vomit,
        4 => 4, r"\[", 255, r"\]"
    )
}
