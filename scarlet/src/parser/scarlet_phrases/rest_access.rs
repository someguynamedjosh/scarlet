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
    env.define_construct(this, CAtomicStructMember(base, AtomicStructMember::Rest));
    this
}

fn uncreate<'a>(
    pc: &ParseContext,
    env: &mut Environment,
    code_arena: &'a Arena<String>,
    uncreate: ConstructId,
    from: &dyn Scope,
) -> Option<Node<'a>> {
    if let Some(asm) =
        downcast_construct::<CAtomicStructMember>(&**env.get_construct_definition(uncreate))
    {
        if asm.1 == AtomicStructMember::Rest {
            let id = asm.0;
            Some(Node {
                phrase: "rest access",
                children: vec![
                    NodeChild::Node(env.vomit(4, true, pc, code_arena, id, from)),
                    NodeChild::Text(".REST"),
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
    format!("{}.REST", src.children[0].as_node().vomit(pc))
}

pub fn phrase() -> Phrase {
    phrase!(
        "rest access",
        Some((create, uncreate)),
        vomit,
        4 => 4, r"\.REST"
    )
}
