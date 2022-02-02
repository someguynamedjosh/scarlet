use typed_arena::Arena;

use crate::{
    constructs::{decision::CDecision, downcast_construct, Construct, ConstructId},
    environment::Environment,
    parser::{phrase::Phrase, Node, NodeChild, ParseContext},
    phrase,
    scope::{SPlain, Scope},
    shared::TripleBool,
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
    let truee = env.get_language_item("true");
    let falsee = env.get_language_item("false");
    env.define_construct(this, CDecision::new(left, right, truee, falsee));
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
        let truee = env.get_language_item("true");
        let falsee = env.get_language_item("false");
        if env.is_def_equal_without_subs(cite.equal(), truee) == TripleBool::True
            && env.is_def_equal_without_subs(cite.unequal(), falsee) == TripleBool::True
        {
            Some(Node {
                phrase: "equal",
                children: vec![
                    NodeChild::Node(env.vomit(127, pc, code_arena, cite.left(), from)),
                    NodeChild::Text("="),
                    NodeChild::Node(env.vomit(127, pc, code_arena, cite.right(), from)),
                ],
            })
        } else {
            None
        }
    } else {
        None
    }
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
