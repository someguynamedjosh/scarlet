use std::ops::ControlFlow;

use typed_arena::Arena;

use crate::{
    constructs::{
        downcast_construct,
        structt::{AtomicStructMember, CAtomicStructMember, CPopulatedStruct},
        ConstructId,
    },
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
    let source = if let Some(asm) =
        env.get_and_downcast_construct_definition::<CAtomicStructMember>(uncreate)
    {
        if asm.1 == AtomicStructMember::Rest {
            Some(asm.0)
        } else {
            None
        }
    } else {
        env.for_each_construct(|env, id| {
            if let Some(cstruct) = env.get_and_downcast_construct_definition::<CPopulatedStruct>(id)
            {
                let cstruct = cstruct.clone();
                if env.is_def_equal(cstruct.get_rest(), uncreate) == TripleBool::True {
                    return ControlFlow::Break(id);
                }
            }
            ControlFlow::Continue(())
        })
    };
    source.map(|id| Node {
        phrase: "rest access",
        children: vec![
            NodeChild::Node(env.vomit(4, pc, code_arena, id, from)),
            NodeChild::Text(".REST"),
        ],
    })
}

fn vomit(pc: &ParseContext, src: &Node) -> String {
    format!("{}.REST", src.children[0].as_node().vomit(pc))
}

pub fn phrase() -> Phrase {
    phrase!(
        "rest access",
        128, 136,
        Some((create, uncreate)),
        vomit,
        4 => 4, r"\.REST"
    )
}
