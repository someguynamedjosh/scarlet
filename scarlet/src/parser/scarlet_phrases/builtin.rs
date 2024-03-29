use crate::{
    definitions::{
        builtin::{Builtin, DBuiltin},
        compound_type::DCompoundType,
    },
    diagnostic::Diagnostic,
    parser::{
        phrase::{CreateContext, CreateResult, Phrase},
        Node,
    },
    phrase,
};

pub fn create(ctx: &mut CreateContext, node: &Node) -> CreateResult {
    assert_eq!(node.children.len(), 4);
    let name = node.children[2].as_ident()?;
    let builtin = match name {
        "is_exactly" => Builtin::IsExactly,
        "if_then_else" => Builtin::IfThenElse,
        "Type" => return Ok(ctx.env.new_defined_item(DCompoundType::god_type())),
        "Union" => Builtin::Union,
        _ => {
            return Err(Diagnostic::new()
                .with_text_error(format!("{} is not the name of any builtin item.", name))
                .with_source_code_block_error(node.position))
        }
    };
    let definition = DBuiltin::new_user_facing(builtin, ctx.env)?;
    Ok(ctx.env.new_defined_item(definition))
}

pub fn phrase() -> Phrase {
    phrase!(
        "builtin",
        128,
        Some((create,)),
        4 => "BUILTIN", r"\(", 255, r"\)"
    )
}
