use crate::{
    diagnostic::Diagnostic,
    environment::{vomit::VomitContext, Environment},
    item::{
        definitions::structt::{DPopulatedStruct, SField, SFieldAndRest},
        Item, ItemDefinition, ItemPtr,
    },
    parser::{
        phrase::{Phrase, UncreateResult},
        util::{self, create_comma_list},
        Node, NodeChild, ParseContext,
    },
    phrase,
    scope::Scope,
};

fn struct_from_fields(
    pc: &ParseContext,
    env: &mut Environment,
    mut fields: Vec<(Option<&str>, &Node)>,
    scope: Box<dyn Scope>,
) -> Result<ItemPtr, Diagnostic> {
    if fields.is_empty() {
        Ok(env.get_language_item("void").unwrap().ptr_clone())
    } else {
        let (label, field) = fields.pop().unwrap();
        let label = label.unwrap_or("").to_owned();
        let this = Item::placeholder_with_scope(format!("struct"), scope);
        let field = field.as_item(pc, env, SFieldAndRest(this.ptr_clone()))?;
        if label.len() > 0 {
            field.set_name(label.clone());
        }
        let body = struct_from_fields(pc, env, fields, Box::new(SField(this.ptr_clone())))?;
        let this_def = DPopulatedStruct::new(body, label, field);
        this.redefine(this_def.clone_into_box());
        Ok(this)
    }
}

fn create(
    pc: &ParseContext,
    env: &mut Environment,
    scope: Box<dyn Scope>,
    node: &Node,
) -> Result<ItemPtr, Diagnostic> {
    assert_eq!(node.children.len(), 3);
    assert_eq!(node.children[0], NodeChild::Text("{"));
    assert_eq!(node.children[2], NodeChild::Text("}"));
    let fields = util::collect_comma_list(&node.children[1]);
    let fields = fields
        .into_iter()
        .map(|field| {
            Ok(if field.phrase == "is" {
                (
                    Some(field.children[0].as_node().as_ident()?),
                    field.children[2].as_node(),
                )
            } else {
                (None, field)
            })
        })
        .collect::<Result<_, _>>()?;
    struct_from_fields(pc, env, fields, scope)
}

fn uncreate<'a>(
    env: &mut Environment,
    ctx: &mut VomitContext<'a, '_>,
    uncreate: ItemPtr,
) -> UncreateResult<'a> {
    let mut maybe_structt = uncreate;
    let mut fields = Vec::new();
    loop {
        let structt = if let Some(structt) = maybe_structt.downcast_definition::<DPopulatedStruct>()
        {
            structt
        } else {
            break;
        };
        let label = ctx.code_arena.alloc(structt.get_tail_label().to_owned());
        let value = structt.get_tail_value().ptr_clone();
        let scope = SFieldAndRest(maybe_structt.ptr_clone());
        let ctx = &mut ctx.with_scope(&scope);
        let rest = structt.get_body().ptr_clone();
        drop(structt);
        maybe_structt = rest;
        let value = env.vomit(255, ctx, value, true);
        if label.len() > 0 {
            fields.push(Node {
                phrase: "is",
                children: vec![
                    NodeChild::Node(Node {
                        phrase: "identifier",
                        children: vec![NodeChild::Text(label)],
                        ..Default::default()
                    }),
                    NodeChild::Text("IS"),
                    NodeChild::Node(value),
                ],
                ..Default::default()
            });
        } else {
            fields.push(value);
        }
    }
    Ok(
        if maybe_structt
            .get_trimmed_equality(&env.get_language_item("void").unwrap())?
            .is_trivial_yes()
        {
            Some(Node {
                phrase: "struct",
                children: vec![
                    NodeChild::Text("{"),
                    create_comma_list(fields),
                    NodeChild::Text("}"),
                ],
                ..Default::default()
            })
        } else {
            None
        },
    )
}

fn vomit(pc: &ParseContext, src: &Node) -> String {
    let contents = src.children[1].vomit(pc);
    if contents.len() > 0 {
        format!("{{ {} }}", contents)
    } else {
        format!("{{}}")
    }
}

pub fn phrase() -> Phrase {
    phrase!(
        "struct",
        128, 120,
        Some((create, uncreate)),
        vomit,
        0 => r"\{", 255, r"\}"
    )
}
