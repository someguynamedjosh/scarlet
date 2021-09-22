use super::context::Context;
use crate::stage2::structure::{Definitions, Item};

/// Returns the item unchanged if definitions is empty.
pub fn with_definitions(ctx: &mut Context, base_item: Item, definitions: Definitions) -> Item {
    if definitions.len() == 0 {
        base_item
    } else {
        let base_id = ctx.environment.insert(base_item);
        Item::Defining {
            base: base_id,
            definitions,
        }
    }
}
