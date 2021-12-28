use typed_arena::Arena;

use crate::{
    constructs::{unique::CUnique, variable::SVariableInvariants, ConstructId},
    environment::Environment,
    parser::{phrase::Phrase, util, Node, NodeChild, ParseContext},
    phrase,
    resolvable::RVariable,
    scope::{SPlain, Scope},
};

fn create<'x>(
    pc: &ParseContext,
    env: &mut Environment<'x>,
    scope: Box<dyn Scope>,
    node: &Node<'x>,
) -> ConstructId {
    assert_eq!(node.children.len(), 4);
    assert_eq!(node.children[1], NodeChild::Text("["));
    assert_eq!(node.children[3], NodeChild::Text("]"));
    let mut invariants = Vec::new();
    let mut depends_on = Vec::new();
    let mut mode = 0;
    let this = env.push_placeholder(scope);
    for arg in util::collect_comma_list(&node.children[2]) {
        if arg.phrase == "identifier" && arg.children == &[NodeChild::Text("DEPENDS_ON")] {
            mode = 1;
        } else if mode == 0 {
            let con = arg.as_construct(pc, env, SVariableInvariants(this));
            invariants.push(con);
        } else {
            let con = arg.as_construct(pc, env, SPlain(this));
            depends_on.push(con);
        }
    }
    let id = env.push_variable();
    let def = RVariable {
        id,
        invariants,
        depends_on,
    };
    env.define_unresolved(this, def);
    this
}

fn uncreate<'a>(
    pc: &ParseContext,
    env: &mut Environment,
    code_arena: &'a Arena<String>,
    uncreate: ConstructId,
    from: ConstructId,
) -> Option<Node<'a>> {
    todo!()
}

pub fn phrase() -> Phrase {
    phrase!(
        "variable",
        Some((create, uncreate)),
        0 => r"\b(VARIABLE|VAR|V)\b" , r"\[", 255, r"\]"
    )
}
