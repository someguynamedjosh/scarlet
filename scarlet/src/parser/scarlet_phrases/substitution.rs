use typed_arena::Arena;

use crate::{
    constructs::{unique::CUnique, ConstructId},
    environment::Environment,
    parser::{phrase::Phrase, Node, NodeChild, ParseContext, util},
    scope::{Scope, SPlain},
    phrase, resolvable::RSubstitution,
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

fn uncreate<'a>(
    pc: &ParseContext,
    env: &mut Environment,
    code_arena: &'a Arena<String>,
    uncreate: ConstructId,
    from: ConstructId,
) -> Option<Node<'a>> {
    None
}

pub fn phrase() -> Phrase {
    phrase!(
        "substitution",
        Some((create, uncreate)),
        4 => 4, r"\[", 255, r"\]"
    )
}
