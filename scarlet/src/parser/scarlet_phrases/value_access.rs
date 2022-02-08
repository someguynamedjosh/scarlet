use std::ops::ControlFlow;

use typed_arena::Arena;

use crate::{
    constructs::{
        downcast_construct,
        structt::{AtomicStructMember, CAtomicStructMember, CPopulatedStruct},
        ConstructId,
    },
    environment::Environment,
    parser::{
        phrase::{Phrase, UncreateResult},
        Node, NodeChild, ParseContext,
    },
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
    env.define_construct(this, CAtomicStructMember(base, AtomicStructMember::Value));
    this
}

fn uncreate<'a>(
    pc: &ParseContext,
    env: &mut Environment,
    code_arena: &'a Arena<String>,
    uncreate: ConstructId,
    from: &dyn Scope,
) -> UncreateResult<'a> {
    let source = if let Some(asm) =
        env.get_and_downcast_construct_definition::<CAtomicStructMember>(uncreate)?
    {
        if asm.1 == AtomicStructMember::Value {
            Some(asm.0)
        } else {
            None
        }
    } else {
        env.for_each_construct(|env, id| {
            if let Ok(Some(cstruct)) =
                env.get_and_downcast_construct_definition::<CPopulatedStruct>(id)
            {
                let cstruct = cstruct.clone();
                if env.is_def_equal_without_subs(cstruct.get_value(), uncreate, 1024)
                    == Ok(TripleBool::True)
                    && from.parent() != Some(id)
                {
                    return ControlFlow::Break(id);
                }
            }
            ControlFlow::Continue(())
        })
    };
    Ok(if let Some(id) = source {
        Some(Node {
            phrase: "value access",
            children: vec![
                NodeChild::Node(env.vomit(4, pc, code_arena, id, from)?),
                NodeChild::Text(".VALUE"),
            ],
        })
    } else {
        None
    })
}

fn vomit(pc: &ParseContext, src: &Node) -> String {
    format!("{}.VALUE", src.children[0].as_node().vomit(pc))
}

pub fn phrase() -> Phrase {
    phrase!(
        "value access",
        128, 136,
        Some((create, uncreate)),
        vomit,
        4 => 4, r"\.VALUE"
    )
}
