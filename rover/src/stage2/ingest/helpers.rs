use super::context::Context;
use crate::{
    shared::{Definitions, ResolvedItem},
    stage2::structure::Item,
};

/// Returns the item unchanged if definitions is empty.
pub fn with_definitions(ctx: &mut Context, base_item: Item, definitions: Definitions) -> Item {
    if definitions.is_empty() {
        base_item
    } else {
        let base_id = ctx.environment.insert(base_item);
        ResolvedItem::Defining {
            base: base_id,
            definitions,
        }
        .into()
    }
}
