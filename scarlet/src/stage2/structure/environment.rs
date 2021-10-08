use super::Item;
use crate::shared::{Id, Pool};

pub type ItemId = Id<AnnotatedItem, 'I'>;
pub type OpaqueId = Id<(), 'O'>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnnotatedItem {
    pub item: Item,
    pub display_requested: bool,
    pub parent_scope: Option<ItemId>,
}

#[derive(Clone, Debug)]
pub struct Environment {
    pub items: Pool<AnnotatedItem, 'I'>,
    pub opaque_value_ids: Pool<(), 'O'>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            items: Pool::new(),
            opaque_value_ids: Pool::new(),
        }
    }

    pub fn new_opaque_value(&mut self) -> OpaqueId {
        self.opaque_value_ids.push(())
    }

    pub fn push_item(&mut self, item: Item) -> ItemId {
        self.items.push(AnnotatedItem {
            item,
            display_requested: false,
            parent_scope: None,
        })
    }

    pub fn mark_displayed(&mut self, item: ItemId) {
        self.items[item].display_requested = true;
    }

    pub fn set_parent_scope(&mut self, item: ItemId, parent_scope: ItemId) {
        self.items[item].parent_scope = Some(parent_scope);
    }
}
