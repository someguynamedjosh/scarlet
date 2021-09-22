use super::context::Context;
use crate::{
    shared::{Definitions, Item},
    stage2::structure::UnresolvedItem,
};

/// Returns the item unchanged if definitions is empty.
pub fn with_definitions(ctx: &mut Context, base_item: UnresolvedItem, definitions: Definitions) -> UnresolvedItem {
    if definitions.is_empty() {
        base_item
    } else {
        let base_id = ctx.environment.insert(base_item);
        Item::Defining {
            base: base_id,
            definitions,
        }
        .into()
    }
}
