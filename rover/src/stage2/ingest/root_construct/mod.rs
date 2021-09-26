mod others;
mod pick;
mod variant;

use others::*;
use pick::*;
use variant::*;

use crate::{
    stage1::structure::construct::Construct,
    stage2::{ingest::Context, structure::UnresolvedItem},
};

pub fn ingest_root_construct(ctx: &mut Context, root: Construct) -> Result<UnresolvedItem, String> {
    match &root.label[..] {
        "identifier" => ingest_identifier(ctx, root),
        "any" => ingest_any_construct(ctx, root),
        "the" => todo!(),
        "i32" => ingest_i32_construct(root),
        "variant" => ingest_variant_construct(ctx, root),
        "pick" => ingest_pick_construct(ctx, root),
        _ => todo!("nice error, unexpected {} construct", root.label),
    }
}
