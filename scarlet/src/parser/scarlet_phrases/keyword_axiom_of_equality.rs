use typed_arena::Arena;

use crate::{
    constructs::{axiom_of_equality::CAxiom, downcast_construct, ConstructId},
    environment::Environment,
    parser::{phrase::Phrase, Node, NodeChild, ParseContext},
    phrase,
    scope::Scope,
};

fn create<'x>(
    _pc: &ParseContext,
    env: &mut Environment<'x>,
    scope: Box<dyn Scope>,
    _node: &Node<'x>,
) -> ConstructId {
    let con = CAxiom::axiom_of_equality(env);
    env.push_construct(con, scope)
}

fn uncreate<'a>(
    pc: &ParseContext,
    env: &mut Environment,
    code_arena: &'a Arena<String>,
    uncreate: ConstructId,
    from: &dyn Scope,
) -> Option<Node<'a>> {
    if let Some(axiom) = downcast_construct::<CAxiom>(&**env.get_construct_definition(uncreate)) {
        Some(Node {
            phrase: "keyword AXIOM_OF_EQUALITY",
            children: vec![NodeChild::Text("AXIOM_OF_EQUALITY")],
        })
    } else {
        None
    }
}

fn vomit(_pc: &ParseContext, _src: &Node) -> String {
    format!("AXIOM")
}

pub fn phrase() -> Phrase {
    phrase!(
        "keyword AXIOM_OF_EQUALITY",
        Some((create, uncreate)),
        vomit,
        0 => r"\bAXIOM_OF_EQUALITY\b"
    )
}
