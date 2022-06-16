use itertools::Itertools;

use crate::{
    diagnostic::Diagnostic,
    environment::{vomit::VomitContext, Environment},
    item::{
        definitions::{substitution::DSubstitution, variable::VariablePtr},
        dependencies::Dependency,
        resolvable::{DResolvable, RSubstitution, UnresolvedItemError},
        ItemDefinition, ItemPtr,
    },
    parser::{
        phrase::{Phrase, UncreateResult},
        util::{self, create_comma_list},
        Node, NodeChild, ParseContext,
    },
    phrase,
    scope::{SPlain, Scope},
    util::PtrExtension,
};

fn create(
    pc: &ParseContext,
    env: &mut Environment,
    scope: Box<dyn Scope>,
    node: &Node,
) -> Result<ItemPtr, Diagnostic> {
    assert_eq!(node.children[1], NodeChild::Text("("));
    assert_eq!(node.children[3], NodeChild::Text(")"));
    assert!(node.children.len() == 4);
    let this = crate::item::Item::placeholder_with_scope(scope);
    let base = node.children[0].as_construct(pc, env, SPlain(this.ptr_clone()))?;
    let mut named_subs = Vec::new();
    let mut anonymous_subs = Vec::new();
    for sub in util::collect_comma_list(&node.children[2]) {
        if sub.phrase == "is" {
            named_subs.push((
                sub.children[0].as_node().as_ident()?.to_owned(),
                sub.children[2].as_construct(pc, env, SPlain(this.ptr_clone()))?,
            ));
        } else {
            anonymous_subs.push(sub.as_construct(pc, env, SPlain(this.ptr_clone()))?);
        }
    }
    this.redefine(
        DResolvable::new(RSubstitution {
            base,
            position: node.position,
            named_subs,
            anonymous_subs,
        })
        .clone_into_box(),
    );
    Ok(this)
}

fn uncreate_substitution<'a>(
    env: &mut Environment,
    ctx: &mut VomitContext<'a, '_>,
    target: VariablePtr,
    value: ItemPtr,
    deps: &mut Vec<Dependency>,
) -> Result<Node<'a>, UnresolvedItemError> {
    let value = env.vomit(254, ctx, value);
    Ok(if deps.get(0).map(|v| v.var == target) == Some(true) {
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
    uncreate: ItemPtr,
) -> UncreateResult<'a> {
    if let Some(csub) = uncreate.downcast_definition::<DSubstitution>() {
        let csub = csub.clone();
        let mut deps = csub
            .base()
            .get_dependencies()
            .into_variables()
            .collect_vec();
        let subs = create_comma_list(
            csub.substitutions()
                .into_iter()
                .map(|(target, value)| {
                    uncreate_substitution(
                        env,
                        ctx,
                        target.ptr_clone(),
                        value.ptr_clone(),
                        &mut deps,
                    )
                })
                .collect::<Result<Vec<_>, _>>()?,
        );
        Ok(Some(Node {
            phrase: "substitution",
            children: vec![
                NodeChild::Node(env.vomit(4, ctx, csub.base().ptr_clone())),
                NodeChild::Text("("),
                subs,
                NodeChild::Text(")"),
            ],
            ..Default::default()
        }))
    } else {
        Ok(None)
    }
}

fn vomit(pc: &ParseContext, src: &Node) -> String {
    format!(
        "{}({})",
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
        4 => 4, r"\(", 255, r"\)"
    )
}
