use typed_arena::Arena;

use crate::{
    constructs::{
        downcast_construct,
        structt::{AtomicStructMember, CAtomicStructMember},
        ConstructId,
    },
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
    assert_eq!(node.children.len(), 2);
    let this = env.push_placeholder(scope);
    let base = node.children[0].as_construct(pc, env, SPlain(this));
    env.define_construct(this, CAtomicStructMember(base, AtomicStructMember::Label));
    this
}

fn uncreate<'a>(
    pc: &ParseContext,
    env: &mut Environment,
    code_arena: &'a Arena<String>,
    uncreate: ConstructId,
    from: &dyn Scope,
) -> Option<Node<'a>> {
    if let Some(asm) = env.get_construct_definition_for_vomiting::<CAtomicStructMember>(uncreate) {
        if asm.1 == AtomicStructMember::Label {
            let id = asm.0;
            Some(Node {
                phrase: "label access",
                children: vec![
                    NodeChild::Node(env.vomit(4, pc, code_arena, id, from)),
                    NodeChild::Text(".LABEL"),
                ],
            })
        } else {
            None
        }
    } else {
        None
    }
}

fn vomit(_pc: &ParseContext, src: &Node) -> String {
    format!("{:#?}", src)
}

pub fn phrase() -> Phrase {
    phrase!(
        "label access",
        128, 128,
        Some((create, uncreate)),
        vomit,
        4 => 4, r"\.LABEL"
    )
}
