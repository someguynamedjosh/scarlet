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
    env.define_construct(this, CAtomicStructMember(base, AtomicStructMember::Value));
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
        downcast_construct::<CAtomicStructMember>(&**env.get_original_construct_definition(uncreate))
    {
        if asm.1 == AtomicStructMember::Value {
            let id = asm.0;
            Some(Node {
                phrase: "value access",
                children: vec![
                    NodeChild::Node(env.vomit(4, true, pc, code_arena, id, from)),
                    NodeChild::Text(".VALUE"),
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
    format!("{}.VALUE", src.children[0].as_node().vomit(pc))
}

pub fn phrase() -> Phrase {
    phrase!(
        "value access",
        Some((create, uncreate)),
        vomit,
        4 => 4, r"\.VALUE"
    )
}
