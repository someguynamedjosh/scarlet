use itertools::Itertools;
use typed_arena::Arena;

use crate::{
    constructs::{
        variable::{CVariable, SVariableInvariants},
        ItemId,
    },
    environment::{vomit::VomitContext, Environment},
    parser::{
        phrase::{Phrase, UncreateResult},
        util::{self, create_comma_list},
        Node, NodeChild, ParseContext,
    },
    phrase,
    resolvable::RVariable,
    scope::{SPlain, SWithParent, Scope},
};

fn create<'x>(
    pc: &ParseContext,
    env: &mut Environment<'x>,
    scope: Box<dyn Scope>,
    node: &Node<'x>,
) -> ItemId {
    assert_eq!(node.children.len(), 4);
    assert_eq!(node.children[1], NodeChild::Text("["));
    assert_eq!(node.children[3], NodeChild::Text("]"));
    let mut invariants = Vec::new();
    let mut dependencies = Vec::new();
    let mut mode = 0;
    let this = env.push_placeholder(scope);
    for arg in util::collect_comma_list(&node.children[2]) {
        if arg.phrase == "identifier" && arg.children == &[NodeChild::Text("DEP")] {
            mode = 1;
        } else if mode == 0 {
            let con = arg.as_construct(pc, env, SVariableInvariants(this));
            invariants.push(con);
        } else {
            let con = arg.as_construct(pc, env, SPlain(this));
            dependencies.push(con);
        }
    }
    let def = RVariable {
        invariants,
        dependencies,
    };
    env.define_unresolved(this, def);
    this
}

fn uncreate<'a>(
    env: &mut Environment,
    ctx: &mut VomitContext<'a, '_>,
    uncreate: ItemId,
) -> UncreateResult<'a> {
    if let Ok(Some(cvar)) = env.get_and_downcast_construct_definition::<CVariable>(uncreate) {
        let cvar = cvar.clone();
        let scope_item = env.push_scope(ctx.scope.dyn_clone());
        let scope_parent = env.dereference(uncreate)?;
        let from = &SWithParent(SVariableInvariants(scope_parent), scope_item);
        let ctx = &mut ctx.with_scope(from);

        let cvar = cvar.clone();
        let var = env.get_variable(cvar.get_id()).clone();
        let invariants = var
            .get_invariants()
            .into_iter()
            .map(|&inv| env.vomit(255, ctx, inv))
            .collect_vec();
        let dependencies = var
            .get_dependencies()
            .into_iter()
            .map(|&dep| env.vomit(255, ctx, dep))
            .collect_vec();
        let mut body = invariants;
        if dependencies.len() > 0 {
            body.push(Node {
                phrase: "identifier",
                children: vec![NodeChild::Text("DEP")],
            });
            let mut depends_on = dependencies;
            body.append(&mut depends_on);
        }
        let node = Node {
            phrase: "variable",
            children: vec![
                NodeChild::Text("VAR"),
                NodeChild::Text("["),
                create_comma_list(body),
                NodeChild::Text("]"),
            ],
        };
        let name = ctx.get_name(env, uncreate, || node);
        Ok(Some(Node {
            phrase: "identifier",
            children: vec![NodeChild::Text(name)],
        }))
    } else {
        Ok(None)
    }
}

fn vomit(pc: &ParseContext, src: &Node) -> String {
    format!("VAR[ {} ]", src.children[2].vomit(pc))
}

pub fn phrase() -> Phrase {
    phrase!(
        "variable",
        128, 128,
        Some((create, uncreate)),
        vomit,
        0 => r"\b(VARIABLE|VAR|V)\b" , r"\[", 255, r"\]"
    )
}
