use crate::{
    shared::{Item, ItemId},
    stage2,
};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ItemDefinition {
    /// A list of all scopes from which info has been requested.
    pub info_requested_in: Vec<ItemId>,
    /// True if this item is a place where other items are defined.
    pub is_scope: bool,
    pub definition: Item,
    pub defined_in: Option<ItemId>,
}

impl ItemDefinition {
    pub fn new(
        is_scope: bool,
        definition: Item,
        defined_in: Option<ItemId>,
    ) -> Self {
        Self {
            info_requested_in: vec![],
            is_scope,
            definition,
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

    pub fn get(&self, id: ItemId) -> &ItemDefinition {
        &self.items[id.0]
    }

    pub fn get_mut(&mut self, id: ItemId) -> &mut ItemDefinition {
        &mut self.items[id.0]
    }

    pub fn insert(&mut self, definition: ItemDefinition) -> ItemId {
        let id = ItemId(self.items.len());
        self.items.push(definition);
        id
    }
}
