use crate::shared::{Item, ItemId};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ItemDefinition {
    /// True when the programmer has requested a diagnostic showing information
    /// about this definition. Contains the scope from which the information was
    /// requested.
    pub info_requested: Option<ItemId>,
    /// True if this item is a place where other items are defined.
    pub is_scope: bool,
    pub definition: Item,
    pub defined_in: Option<ItemId>,
}

impl ItemDefinition {
    pub fn new(item: Item, defined_in: Option<ItemId>) -> Self {
        Self {
            info_requested: None,
            is_scope: false,
            definition: item,
            defined_in,
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

    pub fn mark_info(&mut self, item: ItemId, info_scope: Option<ItemId>) {
        assert!(item.0 < self.items.len());
        self.items[item.0].info_requested = info_scope;
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

    pub fn insert_item(&mut self, item: Item, defined_in: Option<ItemId>) -> ItemId {
        self.insert(ItemDefinition::new(item, defined_in))
    }
}
