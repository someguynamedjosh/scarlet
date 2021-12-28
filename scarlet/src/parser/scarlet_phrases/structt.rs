use typed_arena::Arena;

use crate::{
    constructs::{
        structt::{CPopulatedStruct, SField, SFieldAndRest},
        unique::CUnique,
        ConstructId,
    },
    environment::Environment,
    parser::{phrase::Phrase, util, Node, NodeChild, ParseContext},
    phrase,
    scope::Scope,
};

fn struct_from_fields<'x>(
    pc: &ParseContext,
    env: &mut Environment<'x>,
    mut fields: Vec<(Option<&str>, &Node<'x>)>,
    scope: Box<dyn Scope>,
) -> ConstructId {
    if fields.is_empty() {
        env.get_builtin_item("void")
    } else {
        let (label, field) = fields.remove(0);
        let label = label.unwrap_or("").to_owned();
        let this = env.push_placeholder(scope);
        let field = field.as_construct(pc, env, SFieldAndRest(this));
        let rest = struct_from_fields(pc, env, fields, Box::new(SField(this)));
        let this_def = CPopulatedStruct::new(label, field, rest);
        env.define_construct(this, this_def);
        this
    }
}

fn create<'x>(
    pc: &ParseContext,
    env: &mut Environment<'x>,
    scope: Box<dyn Scope>,
    node: &Node<'x>,
) -> ConstructId {
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
    pc: &ParseContext,
    env: &mut Environment,
    code_arena: &'a Arena<String>,
    uncreate: ConstructId,
    from: ConstructId,
) -> Option<Node<'a>> {
    None
}

pub fn phrase() -> Phrase {
    phrase!(
        "struct",
        Some((create, uncreate)),
        0 => r"\{", 255, r"\}"
    )
}