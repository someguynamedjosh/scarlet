use super::{Item, ItemId};
use crate::shared::{Id, Pool};

#[derive(Clone, Debug)]
pub struct Variable;

#[derive(Clone, Debug)]
pub struct Variant;

pub type OpaqueId = Id<(), 'O'>;

#[derive(Clone, Debug)]
pub struct Environment {
    pub items: Pool<Item, 'I'>,
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
        self.items.push(item)
    }
}
