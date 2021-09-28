use std::collections::HashMap;

use super::shared_items::convert_shared_item;
use crate::{
    shared::{Item, ItemId},
    stage2::structure::{self as stage2},
    stage3::structure::Environment,
};

pub struct Context<'a> {
    pub id_map: HashMap<ItemId, ItemId>,
    pub src: &'a stage2::Environment,
    pub env: &'a mut Environment,
    pub next_unused_id: ItemId,
    pub extra_items: Vec<(Item, Option<ItemId>)>,
}

/// Returns true if calling convert_item(item) will not panic.
pub fn convertible(item: &stage2::UnresolvedItem) -> bool {
    match item {
        stage2::UnresolvedItem::Item(..) | stage2::UnresolvedItem::Member { .. } => false,
        _ => true,
    }
}

impl<'a> Context<'a> {
    pub fn insert_extra_item(&mut self, item: Item, defined_in: Option<ItemId>) -> ItemId {
        for (index, this_item) in self.env.items.iter().enumerate() {
            if this_item.definition == item && this_item.defined_in == defined_in {
                return ItemId(index);
            }
        }
        for (index, this_item) in self.extra_items.iter().enumerate() {
            if this_item.0 == item && this_item.1 == defined_in {
                return ItemId(self.env.items.len() + index);
            }
        }
        let id = self.next_unused_id;
        self.extra_items.push((item, defined_in));
        self.next_unused_id.0 += 1;
        id
    }
}

/// Returns a new item with full_convert_iid applied to all its referenced
/// ids.
pub fn convert_unresolved_item(
    ctx: &mut Context,
    item: &stage2::UnresolvedItem,
) -> Result<Item, String> {
    match item {
        stage2::UnresolvedItem::Just(shared_item) => convert_shared_item(ctx, shared_item),
        stage2::UnresolvedItem::Item(..) | stage2::UnresolvedItem::Member { .. } => {
            panic!("Cannot convert these")
        }
    }
}
