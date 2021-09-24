use crate::shared::{Item, ItemId};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ItemDefinition {
    /// True when the programmer has requested a diagnostic showing information
    /// about this definition.
    pub info_requested: bool,
    /// True if this item is a place where other items are defined.
    pub is_scope: bool,
    pub definition: Item,
}

impl ItemDefinition {
    pub fn new(item: Item) -> Self {
        Self {
            info_requested: false,
            is_scope: false,
            definition: item,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Environment {
    pub items: Vec<ItemDefinition>,
}

impl Environment {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn mark_info(&mut self, item: ItemId) {
        assert!(item.0 < self.items.len());
        self.items[item.0].info_requested = true;
    }

    pub fn mark_as_scope(&mut self, item: ItemId) {
        assert!(item.0 < self.items.len());
        self.items[item.0].is_scope = true;
    }

    pub fn insert(&mut self, definition: ItemDefinition) -> ItemId {
        let id = ItemId(self.items.len());
        self.items.push(definition);
        id
    }

    pub fn insert_item(&mut self, item: Item) -> ItemId {
        self.insert(ItemDefinition::new(item))
    }
}
