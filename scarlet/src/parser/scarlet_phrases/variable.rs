use itertools::Itertools;
use typed_arena::Arena;

use crate::{
    constructs::{
        downcast_construct,
        unique::CUnique,
        variable::{CVariable, SVariableInvariants},
        Construct, ConstructId,
    },
    environment::Environment,
    parser::{
        phrase::Phrase,
        util::{self, create_comma_list},
        Node, NodeChild, ParseContext,
    },
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
    let mut substitutions = Vec::new();
    let mut mode = 0;
    let this = env.push_placeholder(scope);
    for arg in util::collect_comma_list(&node.children[2]) {
        if arg.phrase == "identifier" && arg.children == &[NodeChild::Text("SUB")] {
            mode = 1;
        } else if mode == 0 {
            let con = arg.as_construct(pc, env, SVariableInvariants(this));
            invariants.push(con);
        } else {
            let con = arg.as_construct(pc, env, SPlain(this));
            substitutions.push(con);
        }
    }
    let id = env.push_variable();
    let def = RVariable {
        id,
        invariants,
        substitutions,
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
    if let Some(cvar) = downcast_construct::<CVariable>(&**env.get_construct_definition(uncreate)) {
        let cvar = cvar.clone();
        let invariants = cvar
            .get_invariants()
            .into_iter()
            .map(|&inv| env.vomit(255, pc, code_arena, inv, from))
            .collect_vec();
        let substitutions = cvar
            .get_substitutions()
            .into_iter()
            .map(|&sub| env.vomit(255, pc, code_arena, sub, from))
            .collect_vec();
        let mut body = invariants;
        if substitutions.len() > 0 {
            body.push(Node {
                phrase: "identifier",
                children: vec![NodeChild::Text("SUB")],
            });
            let mut depends_on = substitutions;
            body.append(&mut depends_on);
        }
        Some(Node {
            phrase: "variable",
            children: vec![
                NodeChild::Text("VAR"),
                NodeChild::Text("["),
                create_comma_list(body),
                NodeChild::Text("]"),
            ],
        })
    } else {
        None
    }
}

pub fn phrase() -> Phrase {
    phrase!(
        "variable",
        Some((create, uncreate)),
        0 => r"\b(VARIABLE|VAR|V)\b" , r"\[", 255, r"\]"
    )
}
