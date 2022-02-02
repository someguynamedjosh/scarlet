use typed_arena::Arena;

use crate::{
    constructs::{decision::CDecision, downcast_construct, ConstructId},
    environment::Environment,
    parser::{
        phrase::Phrase,
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
) -> ConstructId {
    assert_eq!(node.children.len(), 4);
    assert_eq!(node.children[0], NodeChild::Text("DECISION"));
    assert_eq!(node.children[1], NodeChild::Text("["));
    assert_eq!(node.children[3], NodeChild::Text("]"));
    let args = util::collect_comma_list(&node.children[2]);
    assert_eq!(args.len(), 4);
    let this = env.push_placeholder(scope);

    let left = args[0].as_construct(pc, env, SPlain(this));
    let right = args[1].as_construct(pc, env, SPlain(this));
    let equal = args[2].as_construct(pc, env, SPlain(this));
    let unequal = args[3].as_construct(pc, env, SPlain(this));
    env.define_construct(this, CDecision::new(left, right, equal, unequal));
    this
}

fn uncreate<'a>(
    pc: &ParseContext,
    env: &mut Environment,
    code_arena: &'a Arena<String>,
    uncreate: ConstructId,
    from: &dyn Scope,
) -> Option<Node<'a>> {
    if let Some(cite) = env.get_and_downcast_construct_definition::<CDecision>(uncreate) {
        let cite = cite.clone();
        Some(Node {
            phrase: "decision",
            children: vec![
                NodeChild::Text("DECISION"),
                NodeChild::Text("["),
                create_comma_list(vec![
                    env.vomit(255, pc, code_arena, cite.left(), from),
                    env.vomit(255, pc, code_arena, cite.right(), from),
                    env.vomit(255, pc, code_arena, cite.equal(), from),
                    env.vomit(255, pc, code_arena, cite.unequal(), from),
                ]),
                NodeChild::Text("]"),
            ],
        })
    } else {
        None
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
