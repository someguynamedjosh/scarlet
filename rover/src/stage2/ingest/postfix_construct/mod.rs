use self::other::ingest_non_defining_postfix_construct;
use super::{context::Context, expression::ingest_expression};
use crate::{
    stage1::structure::{construct::Construct, expression::Expression},
    stage2::{ingest::definitions::process_definitions, structure::Item},
};

mod from;
mod other;

pub fn ingest_postfix_construct(
    ctx: &mut Context,
    post: Construct,
    remainder: Expression,
) -> Result<Item, String> {
    if post.label == "defining" {
        ingest_defining_construct(ctx, post, remainder)
    } else {
        let base_id = ingest_expression(&mut ctx.child(), remainder)?;
        ingest_non_defining_postfix_construct(ctx, base_id, post)
    }
}

fn ingest_defining_construct(
    ctx: &mut Context,
    post: Construct,
    remainder: Expression,
) -> Result<Item, String> {
    let statements = post.expect_statements("defining")?.to_owned();
    let body = process_definitions(&mut ctx.child(), statements, vec![])?;
    let mut child_ctx = ctx.child().with_additional_scope(&body);
    let base_id = ingest_expression(&mut child_ctx, remainder)?;
    Ok(Item::Defining {
        base: base_id,
        definitions: body,
    })
}
