use itertools::Itertools;
use typed_arena::Arena;

use crate::{
    constructs::{
        substitution::CSubstitution,
        variable::{Dependency, VariableId},
        ConstructId,
    },
    environment::{Environment, UnresolvedConstructError},
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
) -> ConstructId {
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
    pc: &ParseContext,
    env: &mut Environment,
    code_arena: &'a Arena<String>,
    target: VariableId,
    value: ConstructId,
    deps: &mut Vec<Dependency>,
    from: &dyn Scope,
) -> Result<Node<'a>, UnresolvedConstructError> {
    let value = env.vomit(254, pc, code_arena, value, from)?;
    Ok(if deps.get(0).map(|v| v.id == target) == Some(true) {
        deps.remove(0);
        value
    } else {
        Node {
            phrase: "is",
            children: vec![
                NodeChild::Node(env.vomit_var(pc, code_arena, target, from)?),
                NodeChild::Text("IS"),
                NodeChild::Node(value),
            ],
        }
    })
}

fn uncreate<'a>(
    pc: &ParseContext,
    env: &mut Environment,
    code_arena: &'a Arena<String>,
    uncreate: ConstructId,
    from: &dyn Scope,
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
                .map(|(target, value)| {
                    uncreate_substitution(pc, env, code_arena, *target, *value, &mut deps, from)
                })
                .collect::<Result<Vec<_>, _>>()?,
        );
        Ok(Some(Node {
            phrase: "substitution",
            children: vec![
                NodeChild::Node(env.vomit(4, pc, code_arena, csub.base(), from)?),
                NodeChild::Text("["),
                subs,
                NodeChild::Text("]"),
            ],
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
