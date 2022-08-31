use itertools::Itertools;

use crate::{
    diagnostic::Diagnostic,
    environment::{vomit::VomitContext, Environment},
    item::{
        definitions::{
            unique::DUnique,
            variable::{DVariable, SVariableInvariants, VariableOrder},
        },
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

fn create(
    pc: &ParseContext,
    env: &mut Environment,
    scope: Box<dyn Scope>,
    node: &Node,
) -> Result<ItemPtr, Diagnostic> {
    assert_eq!(node.children.len(), 4);
    assert_eq!(node.children[1], NodeChild::Text("("));
    assert_eq!(node.children[3], NodeChild::Text(")"));
    let mut dependencies = Vec::new();
    let mut statement = None;
    let mut order = VariableOrder::new(
        128,
        node.position.file_index() as _,
        node.position.range().start as _,
    );
    let mut mode = 0;
    let this = Item::placeholder_with_scope(format!("variable"), scope.dyn_clone());
    for arg in util::collect_comma_list(&node.children[2]) {
        if arg.phrase == "identifier" && arg.children == &[NodeChild::Text("DEP")] {
            mode = 1;
        } else if arg.phrase == "identifier" && arg.children == &[NodeChild::Text("ORD")] {
            mode = 2;
        } else if mode == 0 {
            if statement.is_none() {
                statement = Some((
                    arg.as_item(pc, env, SPlain(this.ptr_clone()))?,
                    arg.vomit(pc),
                ));
            } else {
                return Err(Diagnostic::new()
                    .with_text_error(format!("Only one theorem is allowed."))
                    .with_source_code_block_error(arg.position));
            }
        } else if mode == 1 {
            let con = arg.as_item(pc, env, SPlain(this.ptr_clone()))?;
            dependencies.push(con);
        } else if mode == 2 {
            let text = arg.as_ident()?;
            order.major_order = text
                .parse()
                .expect("TODO: Nice error, expected order to be a number between 0 and 255");
            mode = 0
        }
    }
    let (statement, statement_text) = if let Some(statement) = statement {
        statement
    } else {
        todo!("nice error")
    };
    let item = DVariable::new_theorem(statement, statement_text, dependencies, order, scope);
    Ok(item)
}

fn uncreate<'a>(
    env: &mut Environment,
    ctx: &mut VomitContext<'a, '_>,
    uncreate: ItemPtr,
) -> UncreateResult<'a> {
    if let Some(cvar) = uncreate.downcast_definition::<DVariable>() {
        if let Some(statement) = cvar.get_variable().borrow().required_theorem() {
            let cvar = cvar.clone();
            let scope_item =
                Item::new_boxed(DUnique::new().clone_into_box(), ctx.scope.dyn_clone());
            let scope_parent = uncreate.dereference();
            let from = &SWithParent(SVariableInvariants(scope_parent), scope_item);
            let ctx = &mut ctx.with_scope(from);

            let cvar = cvar.clone();
            let var = cvar.get_variable().borrow();
            let dependencies = var
                .get_dependencies()
                .into_iter()
                .map(|dep| env.vomit(255, ctx, dep.ptr_clone(), true))
                .collect_vec();
            let mut body = vec![env.vomit(255, ctx, statement.ptr_clone(), true)];
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
                phrase: "any_proof_of",
                children: vec![
                    NodeChild::Text("ANY_PROOF"),
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
    } else {
        Ok(None)
    }
}

fn vomit(pc: &ParseContext, src: &Node) -> String {
    format!("ANY_PROOF({})", src.children[2].vomit(pc))
}

pub fn phrase() -> Phrase {
    phrase!(
        "any_proof_of",
        128, 128,
        Some((create, uncreate)),
        vomit,
        0 => r"\b(ANY_PROOF|ANY_PROOF|ANPR)\b" , r"\(", 255, r"\)"
    )
}
