use std::rc::Rc;

use crate::{
    definitions::compound_type::{DCompoundType, Type},
    parser::{
        phrase::{CreateContext, CreateResult, Phrase},
        util::collect_comma_list,
        Node,
    },
    phrase,
};

pub fn create(ctx: &mut CreateContext, node: &Node) -> CreateResult {
    assert_eq!(node.children.len(), 4);
    let mut fields = Vec::new();
    for child in collect_comma_list(&node.children[2]) {
        if let Some(is) = child.as_is() {
            let (label, value) = is?;
            fields.push((label.to_owned(), value.as_item(ctx)?));
        } else {
            fields.push((String::new(), child.as_item(ctx)?));
        }
    }
    let id = ctx.env.new_item();
    let def = DCompoundType::new_single(Rc::new(Type::UserType {
        type_id: Rc::new(()),
        fields,
    }));
    ctx.env.define_item(id, def);
    Ok(id)
}

pub fn phrase() -> Phrase {
    phrase!(
        "new type",
        128,
        Some((create,)),
        4 => "NEW_TYPE", r"\(", 255, r"\)"
    )
}
