use typed_arena::Arena;

use crate::{
    constructs::{downcast_construct, equal::CEqual, unique::CUnique, ConstructId},
    environment::Environment,
    parser::{phrase::Phrase, Node, NodeChild, ParseContext},
    phrase,
    scope::{SPlain, Scope},
};

fn create<'x>(
    pc: &ParseContext,
    env: &mut Environment<'x>,
    scope: Box<dyn Scope>,
    node: &Node<'x>,
) -> ConstructId {
    assert_eq!(node.children.len(), 3);
    assert_eq!(node.children[1], NodeChild::Text("="));
    let this = env.push_placeholder(scope);
    let left = node.children[0].as_construct(pc, env, SPlain(this));
    let right = node.children[2].as_construct(pc, env, SPlain(this));
    env.define_construct(this, CEqual::new(left, right));
    this
}

fn uncreate<'a>(
    pc: &ParseContext,
    env: &mut Environment,
    code_arena: &'a Arena<String>,
    uncreate: ConstructId,
    from: ConstructId,
) -> Option<Node<'a>> {
    if let Some(ceq) = downcast_construct::<CEqual>(&**env.get_construct_definition(uncreate)) {
        let ceq = ceq.clone();
        Some(Node {
            phrase: "equal operator",
            children: vec![
                NodeChild::Node(env.vomit(63, pc, code_arena, ceq.left(), from)),
                NodeChild::Text("="),
                NodeChild::Node(env.vomit(63, pc, code_arena, ceq.right(), from)),
            ],
        })
    } else {
        None
    }
}

pub fn phrase() -> Phrase {
    phrase!(
        "equal operator",
        Some((create, uncreate)),
        65 => 65, r"=", 65
    )
}
