use itertools::Itertools;

use crate::{
    environment::{vomit::VomitContext, Environment},
    item::{
        definitions::{
            unique::DUnique,
            variable::{DVariable, SVariableInvariants, VariableOrder},
        },
        resolvable::{DResolvable, RVariable},
        Item, ItemDefinition, ItemPtr,
    },
    parser::{
        phrase::{Phrase, UncreateResult},
        util::{self, create_comma_list},
        Node, NodeChild, ParseContext,
    },
    phrase,
    scope::{SPlain, SWithParent, Scope},
};

fn create(pc: &ParseContext, env: &mut Environment, scope: Box<dyn Scope>, node: &Node) -> ItemPtr {
    assert_eq!(node.children.len(), 4);
    assert_eq!(node.children[1], NodeChild::Text("("));
    assert_eq!(node.children[3], NodeChild::Text(")"));
    let mut invariants = Vec::new();
    let mut dependencies = Vec::new();
    let mut order =
        VariableOrder::new(128, node.position.file_index, node.position.start_char as _);
    let mut mode = 0;
    let this = crate::item::Item::placeholder_with_scope(scope.dyn_clone());
    for arg in util::collect_comma_list(&node.children[2]) {
        if arg.phrase == "identifier" && arg.children == &[NodeChild::Text("DEP")] {
            mode = 1;
        } else if arg.phrase == "identifier" && arg.children == &[NodeChild::Text("ORD")] {
            mode = 2;
        } else if mode == 0 {
            let con = arg.as_construct(pc, env, SVariableInvariants(this.ptr_clone()));
            invariants.push(con);
        } else if mode == 1 {
            let con = arg.as_construct(pc, env, SPlain(this.ptr_clone()));
            dependencies.push(con);
        } else if mode == 2 {
            let text = arg.as_ident();
            order.major_order = text
                .parse()
                .expect("TODO: Nice error, expected order to be a number between 0 and 255");
            mode = 0
        }
    }
    let def = RVariable {
        invariants,
        dependencies,
        order,
    };
    this.redefine(DResolvable::new(def).clone_into_box());
    this
}

fn uncreate<'a>(
    env: &mut Environment,
    ctx: &mut VomitContext<'a, '_>,
    uncreate: ItemPtr,
) -> UncreateResult<'a> {
    if let Some(cvar) = uncreate.downcast_definition::<DVariable>() {
        let cvar = cvar.clone();
        let scope_item = Item::new_boxed(DUnique::new().clone_into_box(), ctx.scope.dyn_clone());
        let scope_parent = uncreate.dereference();
        let from = &SWithParent(SVariableInvariants(scope_parent), scope_item);
        let ctx = &mut ctx.with_scope(from);

        let cvar = cvar.clone();
        let var = cvar.get_variable().borrow();
        let invariants = var
            .get_invariants()
            .into_iter()
            .map(|inv| env.vomit(255, ctx, inv.ptr_clone()))
            .collect_vec();
        let dependencies = var
            .get_dependencies()
            .into_iter()
            .map(|dep| env.vomit(255, ctx, dep.ptr_clone()))
            .collect_vec();
        let mut body = invariants;
        if dependencies.len() > 0 {
            body.push(Node {
                phrase: "identifier",
                children: vec![NodeChild::Text("DEP")],
                ..Default::default()
            });
            let mut depends_on = dependencies;
            body.append(&mut depends_on);
        }
        let node = Node {
            phrase: "variable",
            children: vec![
                NodeChild::Text("VAR"),
                NodeChild::Text("("),
                create_comma_list(body),
                NodeChild::Text(")"),
            ],
            ..Default::default()
        };
        let name = ctx.get_name(env, uncreate.ptr_clone(), || node);
        Ok(Some(Node {
            phrase: "identifier",
            children: vec![NodeChild::Text(name)],
            ..Default::default()
        }))
    } else {
        Ok(None)
    }
}

fn vomit(pc: &ParseContext, src: &Node) -> String {
    format!("VAR({})", src.children[2].vomit(pc))
}

pub fn phrase() -> Phrase {
    phrase!(
        "variable",
        128, 128,
        Some((create, uncreate)),
        vomit,
        0 => r"\b(VARIABLE|VAR|V)\b" , r"\(", 255, r"\)"
    )
}
