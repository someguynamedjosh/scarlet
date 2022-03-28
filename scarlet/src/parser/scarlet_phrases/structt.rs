use typed_arena::Arena;

use crate::{
    constructs::{
        structt::{CPopulatedStruct, SField, SFieldAndRest},
        ItemId,
    },
    environment::{discover_equality::Equal, vomit::VomitContext, Environment},
    parser::{
        phrase::{Phrase, UncreateResult},
        util::{self, create_comma_list},
        Node, NodeChild, ParseContext,
    },
    phrase,
    scope::Scope,
};

fn struct_from_fields<'x>(
    pc: &ParseContext,
    env: &mut Environment<'x>,
    mut fields: Vec<(Option<&str>, &Node<'x>)>,
    scope: Box<dyn Scope>,
) -> ItemId {
    if fields.is_empty() {
        env.get_language_item("void")
    } else {
        let (label, field) = fields.remove(0);
        let label = label.unwrap_or("").to_owned();
        let this = env.push_placeholder(scope);
        let field = field.as_construct(pc, env, SFieldAndRest(this));
        if label.len() > 0 {
            env.set_name(field, label.clone());
        }
        let rest = struct_from_fields(pc, env, fields, Box::new(SField(this)));
        let this_def = CPopulatedStruct::new(label, field, rest);
        env.define_item(this, this_def);
        this
    }
}

fn create<'x>(
    pc: &ParseContext,
    env: &mut Environment<'x>,
    scope: Box<dyn Scope>,
    node: &Node<'x>,
) -> ItemId {
    assert_eq!(node.children.len(), 3);
    assert_eq!(node.children[0], NodeChild::Text("{"));
    assert_eq!(node.children[2], NodeChild::Text("}"));
    let fields = util::collect_comma_list(&node.children[1]);
    let fields = fields
        .into_iter()
        .map(|field| {
            if field.phrase == "is" {
                (
                    Some(field.children[0].as_node().as_ident()),
                    field.children[2].as_node(),
                )
            } else {
                (None, field)
            }
        })
        .collect();
    struct_from_fields(pc, env, fields, scope)
}

fn uncreate<'a>(
    env: &mut Environment,
    ctx: &mut VomitContext<'a, '_>,
    uncreate: ItemId,
) -> UncreateResult<'a> {
    let mut maybe_structt = uncreate;
    let mut fields = Vec::new();
    while let Some(structt) =
        env.get_and_downcast_construct_definition::<CPopulatedStruct>(maybe_structt)?
    {
        let label = ctx.code_arena.alloc(structt.get_label().to_owned());
        let value = structt.get_value();
        let scope = SFieldAndRest(maybe_structt);
        let ctx = &mut ctx.with_scope(&scope);
        maybe_structt = structt.get_rest();
        let value = env.vomit(255, ctx, value);
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
        if env.discover_equal(maybe_structt, env.get_language_item("void"), 2)? == Equal::yes() {
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
